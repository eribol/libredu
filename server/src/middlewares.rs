use std::pin::Pin;
use tide::{Request, Next, Response, StatusCode, Result};
use crate::AppState;
use std::future::Future;
use crate::request::SchoolAuth;

pub fn school_auth<'a>(
    mut request: Request<AppState>,
    next: Next<'a, AppState>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        use crate::request::Auth;
        if let Some(school) = request.get_school().await {
            let role = request.get_school_auth().await;
            let school_auth = SchoolAuth { school, role };
            request.set_ext(school_auth.clone());
            Ok(next.run(request).await)
        }
        else {
            Ok(Response::new(StatusCode::Unauthorized))
        }
    })
}

pub fn group_auth<'a>(
    mut request: Request<AppState>,
    next: Next<'a, AppState>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        use crate::request::Auth;
        if let Some(_group) = request.get_group().await {
            Ok(next.run(request).await)
        }
        else {
            Ok(Response::new(StatusCode::Unauthorized))
        }
    })
}

pub fn user_auth<'a>(
    mut request: Request<AppState>,
    next: Next<'a, AppState>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
    Box::pin(async {
        use crate::request::Auth;
        if let Some(user) = request.user().await {
            request.set_ext(user);
            Ok(next.run(request).await)
        } else {
            Ok(Response::new(StatusCode::Unauthorized))
        }
    })
}