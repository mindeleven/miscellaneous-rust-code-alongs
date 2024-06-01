/// example from Wikipedia: Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==
/// username: Aladdin, password: open sesame
/// https://en.wikipedia.org/wiki/Basic_access_authentication
/// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='

use rocket::{
    http::Status, request::{
        FromRequest, Outcome, Request
    }
};

#[allow(dead_code)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

/// factory functions are taken from the course author's github repo
impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // If exactly username & password pair are present
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());
        
        /* // println!("username: {}, password: {}", username, password);
        // username: Aladdin, password: open sesame
        if username != "Aladdin" && password != "open sesame" {
            return None;
        } */

        Some(BasicAuth {
            username,
            password
        })
    }
}

// implementing trait for basic authentication
// https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest
// trait implemented by request guards to derive a value from incoming requests
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // get_one() -> Returns the first value stored for the header with name name if there is one
        let auth_header = request.headers().get_one("Authorization");
        /* 
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                return Outcome::Success(auth)
            }
        }
        */
        // now with username / password check
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                // Check username / pass
                if auth.username == String::from("Aladdin") && auth.password == String::from("open sesame") {
                    return Outcome::Success(auth)
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}