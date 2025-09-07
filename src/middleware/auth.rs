// src/middleware/auth.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
    error::ErrorUnauthorized,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use crate::services::{auth_service::AuthService, opa_service::{OpaService, OpaResource}};
use std::collections::HashMap;

pub struct AuthMiddleware {
    auth_service: Rc<AuthService>,
    opa_service: Rc<OpaService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: AuthService, opa_service: OpaService) -> Self {
        Self {
            auth_service: Rc::new(auth_service),
            opa_service: Rc::new(opa_service),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
            auth_service: self.auth_service.clone(),
            opa_service: self.opa_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    auth_service: Rc<AuthService>,
    opa_service: Rc<OpaService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let auth_service = self.auth_service.clone();
        let opa_service = self.opa_service.clone();

        Box::pin(async move {
            // Extract token from header
            let token = req
                .headers()
                .get("Authorization")
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer "))
                .ok_or_else(|| ErrorUnauthorized("Missing or invalid token"))?;

            // Verify token
            let claims = auth_service
                .verify_token(token)
                .await
                .map_err(|_| ErrorUnauthorized("Invalid token"))?;

            println!("{:?}", claims);
            // Check API permission with OPA
            let path = req.path();
            let method = req.method().as_str();

            let resource = OpaResource {
                resource_type: "api_endpoint".to_string(),
                resource_id: Some(path.to_string()),
                partner_id: Some(claims.partner_id),
                attributes: HashMap::new(),
            };

            println!("{:?}", resource);

            let allowed = opa_service
                .check_permission(&claims, method, resource)
                .await
                .map_err(|_| ErrorUnauthorized("Permission check failed"))?;

            if !allowed {
                return Err(ErrorUnauthorized("Access denied"));
            }

            // Store claims in request extensions
            req.extensions_mut().insert(claims);

            service.call(req).await
        })
    }
}
