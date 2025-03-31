use tower_cookies::{Cookie, Cookies};
use super::Result;
use tracing::debug;

use crate::crypt::jwt_token::JwtToken;

pub const AUTH_COOKIE_NAME: &str = "auth";

pub fn set_jwt_cookie(cookies: &Cookies, user_id: i64, token_salt: &str) -> Result<()> {
    
    let jwt_token = JwtToken::new(user_id, token_salt)?;

    debug!("JWT TOKEN SET: {}", jwt_token);

    let mut auth_cookie = Cookie::new(AUTH_COOKIE_NAME, jwt_token.to_string());
    auth_cookie.set_http_only(true);
    auth_cookie.set_path("/");
    
    cookies.add(auth_cookie);
    
    Ok(())
}

pub fn delete_jwt_cookie(cookies: &Cookies) -> Result<()> {
    let mut jwt_cookie = Cookie::from(AUTH_COOKIE_NAME);
    jwt_cookie.set_path("/");
    
    cookies.remove(jwt_cookie);
    Ok(())
}