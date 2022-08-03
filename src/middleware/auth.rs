use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use crate::models::auth::user::User;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};

pub struct AuthGuard;

impl<S> Transform<S, ServiceRequest> for AuthGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthGuardMiddleware { service }))
    }
}

pub struct AuthGuardMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match is_logged(&req) {
            Ok(user) => {
                req.extensions_mut().insert(user);
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(err) => Box::pin(async move {
                Ok(ServiceResponse::new(
                    req.into_parts().0,
                    HttpResponse::Unauthorized().body(err),
                ))
            }),
        }
    }

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
}

fn is_logged(req: &ServiceRequest) -> Result<User, String> {
    let header = match &req.headers().get("Authorization") {
        Some(head) => match head.to_str().ok() {
            Some(x) => x.to_string(),
            None => return Err(String::from("Couldn't parse the header.")),
        },
        None => return Err(String::from("Couldn't get headers.")),
    };
    let mut header = header.split_whitespace();

    let auth_type = header.next();

    if Some("Bearer") == auth_type {
        bearer_auth(match header.next() {
            Some(val) => val,
            None => "",
        })
    } else {
        Err(String::from("Invalid authentication method"))
    }
}

fn bearer_auth(token: &str) -> Result<User, String> {
    match crate::services::jwt::verify(token.to_string()) {
        Ok(user) => Ok(user),
        Err(_e) => Err("JWT error".to_string()),
    }
}