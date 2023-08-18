use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::future::{ready, Ready};

use super::jwt::Claims;

pub struct JWT;

impl<S, B> Transform<S, ServiceRequest> for JWT
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckLoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let req = request.headers().clone();

        match req.get("token") {
            Some(tokn) => {
                let decode_ke = DecodingKey::from_secret("SECRET_KEY".as_bytes());

                let token = tokn.to_str().unwrap();
                match decode::<Claims>(&token, &decode_ke, &Validation::default()) {
                    Ok(claim) => {
                        let res = self.service.call(request);

                        let expires = chrono::NaiveDateTime::from_timestamp_opt(
                            (claim.claims.exp as i64).try_into().unwrap(),
                            0,
                        )
                        .unwrap();

                        let timezone: Tz = "America/Lima".parse().unwrap();
                        let us = Utc.from_utc_datetime(&expires).with_timezone(&timezone);
                        let now = Utc::now().with_timezone(&timezone);

                        if now >= us {
                            return Box::pin(async {
                                Err(actix_web::error::ErrorUnauthorized(
                                    json!({"message": "El token ha expirado"}),
                                ))
                            });
                        }

                        return Box::pin(async move {
                            res.await.map(ServiceResponse::map_into_left_body)
                        });
                    }
                    Err(_s) => {
                        let (reqs, _) = request.into_parts();
                        let res = HttpResponse::Unauthorized()
                            .json(ResponseBody {
                                message: "token invalido".to_string(),
                                code: None,
                            })
                            .map_into_right_body();

                        return Box::pin(async { Ok(ServiceResponse::new(reqs, res)) });
                    }
                }
            }
            None => {
                let (reqs, _) = request.into_parts();
                let res = HttpResponse::Unauthorized()
                    .json(ResponseBody {
                        message: "n".to_string(),
                        code: None,
                    })
                    .map_into_right_body();

                return Box::pin(async { Ok(ServiceResponse::new(reqs, res)) });
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    pub message: String,
    pub code: Option<String>,
}
