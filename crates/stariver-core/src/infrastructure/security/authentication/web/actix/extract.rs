use std::future::{ready, Ready};

use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};

use crate::infrastructure::security::authentication::core::principal_extract::Extractor;
use crate::infrastructure::security::authentication::user_principal::User;
use crate::infrastructure::security::authentication::web::actix::error::ErrUnauthorized;

impl Extractor for User {
    type Payload = HttpRequest;
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_payload(payload: &Self::Payload) -> Self::Future {
        Self::extract(payload)
    }
}

impl FromRequest for User {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        let cookie = req.cookie("id");
        match cookie {
            None => {
                let unauthorized = ErrUnauthorized {};
                ready(Err(unauthorized.into()))
            }
            Some(cookie) => {
                let value = cookie.value();
                println!("--->{}", value);
                let result = serde_json::from_str::<User>(value).map_err(|e| {
                    println!("####{}", e);
                    let unauthorized = ErrUnauthorized {};
                    unauthorized.into()
                });
                ready(result)
            }
        }
    }
}
