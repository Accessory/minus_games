echo -n "$1" | argon2 $(openssl rand -base64 15) -id -e