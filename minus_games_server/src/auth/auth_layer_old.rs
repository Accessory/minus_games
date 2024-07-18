fn main () {
    let arc_user: Option<Arc<User>> = match session_id_option.as_ref() {
        None => match user_option {
            None => {
                let new_arc_user = Arc::new(user_handler.get_default_user());
                let key = Uuid::new_v4();
                session_manager
                    .write()
                    .await
                    .sessions
                    .insert(key, new_arc_user.clone());
                cookie = Some(Cookie::new(COOKIES_SESSION_NAME, key.to_string()));
                Some(new_arc_user)
            }
            Some(up) => {
                let user = user_handler.authorize_user_by_username_password(&up.0, &up.1);
                match user {
                    None => None,
                    Some(user) => {
                        let new_arc_user = Arc::new(user);
                        let key = Uuid::new_v4();
                        session_manager
                            .write()
                            .await
                            .sessions
                            .insert(key, new_arc_user.clone());
                        cookie = Some(Cookie::new(COOKIES_SESSION_NAME, key.to_string()));
                        Some(new_arc_user)
                    }
                }
            }
        },
        Some(session_id) => match session_manager.read().await.sessions.get(session_id) {
            None => match user_option {
                None => {
                    let new_arc_user = Arc::new(user_handler.get_default_user());
                    let key = Uuid::new_v4();
                    session_manager
                        .write()
                        .await
                        .sessions
                        .insert(key, new_arc_user.clone());
                    cookie = Some(Cookie::new(COOKIES_SESSION_NAME, key.to_string()));
                    Some(new_arc_user)
                }
                Some(up) => {
                    let user =
                        user_handler.authorize_user_by_username_password(&up.0, &up.1);
                    match user {
                        None => None,
                        Some(user) => {
                            let new_arc_user = Arc::new(user);
                            let key = Uuid::new_v4();
                            session_manager
                                .write()
                                .await
                                .sessions
                                .insert(key, new_arc_user.clone());
                            cookie =
                                Some(Cookie::new(COOKIES_SESSION_NAME, key.to_string()));
                            Some(new_arc_user)
                        }
                    }
                }
            },
            Some(user) => Some(user.clone()),
        },
    };


    if let Some(id) = new_session_uuid {

        let cookie = Cookie::
        response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(cookie_string.as_str()).unwrap(),
        );
    }
}