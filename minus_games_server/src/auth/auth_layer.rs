use crate::auth::session_manager::{session_id_from_request, SessionManager, COOKIES_SESSION_NAME};
use crate::auth::user_handler::UserHandler;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tower::{Layer, Service};
use tracing::trace;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthLayer {
    session_manager: Arc<RwLock<SessionManager>>,
    user_handler: Arc<UserHandler>,
    clear_sessions: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl AuthLayer {
    pub fn new(
        user_handler: Arc<UserHandler>,
        session_manager: Arc<RwLock<SessionManager>>,
        clear_sessions: Arc<RwLock<Option<JoinHandle<()>>>>,
    ) -> Self {
        Self {
            user_handler,
            session_manager,
            clear_sessions,
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            user_handler: self.user_handler.clone(),
            session_manager: self.session_manager.clone(),
            clear_sessions: self.clear_sessions.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    user_handler: Arc<UserHandler>,
    session_manager: Arc<RwLock<SessionManager>>,
    clear_sessions: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static + Clone,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let user_option = self.user_handler.get_authorization_from_request(&request);

        // Session
        let session_id_option = session_id_from_request(&request);
        let user_handler = self.user_handler.clone();
        let session_manager = self.session_manager.clone();
        let clone = self.inner.clone();
        let mut inner_clone = std::mem::replace(&mut self.inner, clone);
        let clear_sessions = self.clear_sessions.clone();

        // info!("session_id_option: {session_id_option:?}, user_option: {user_option:?}");
        Box::pin(async move {
            let (new_session_uuid, arc_user) = match (session_id_option, user_option) {
                (None, None) => (
                    Some(Uuid::new_v4()),
                    Some(Arc::new(user_handler.get_default_user())),
                ),
                (Some(session_id), None) => {
                    match session_manager.read().await.sessions.get(&session_id) {
                        None => (
                            Some(session_id),
                            Some(Arc::new(user_handler.get_default_user())),
                        ),
                        Some(user) => (None, Some(user.clone())),
                    }
                }
                (None, Some(user_option)) => {
                    let arc_user = user_handler
                        .authorize_user_by_username_password(&user_option.0, &user_option.1)
                        .map(Arc::new);
                    (Some(Uuid::new_v4()), arc_user)
                }
                (Some(session_id), Some(user_option)) => {
                    match session_manager.read().await.sessions.get(&session_id) {
                        None => {
                            let arc_user = user_handler
                                .authorize_user_by_username_password(&user_option.0, &user_option.1)
                                .map(Arc::new);
                            (Some(session_id), arc_user)
                        }
                        Some(user) => (None, Some(user.clone())),
                    }
                }
            };

            if arc_user.is_none() {
                return Ok((StatusCode::UNAUTHORIZED, "User is unauthorized.").into_response());
            }

            let user = arc_user.unwrap();
            request.extensions_mut().insert(user.clone());

            let mut response: Response = inner_clone.call(request).await?;

            if let Some(id) = new_session_uuid {
                trace!("Set up new session for {}. Id: {}", user.username, id);
                response.headers_mut().insert(
                    SET_COOKIE,
                    HeaderValue::from_str(&format!("{COOKIES_SESSION_NAME}={id}; path=/")).unwrap(),
                );
                session_manager.write().await.sessions.insert(id, user);
            }

            if let Some(handle) =
                clear_sessions
                    .write()
                    .await
                    .replace(tokio::task::spawn(async move {
                        tokio::time::sleep(Duration::from_mins(10)).await;
                        trace!("Clear all sessions");
                        session_manager.write().await.sessions.clear();
                    }))
            {
                handle.abort();
            }

            Ok(response)
        })
    }
}
