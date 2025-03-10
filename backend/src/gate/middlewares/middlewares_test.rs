#![allow(unused_imports)]

#[cfg(test)]
mod tests {
   use reqwest::RequestBuilder;
   use axum::{
      body::Body,
      extract::FromRequestParts,
      http::{Request, StatusCode, request::Parts},
      middleware::{self, Next},
      response::{IntoResponse, Response},
      routing::get,
      Extension,
      Router
   };
   use tower::ServiceBuilder;
   use tower::ServiceExt;
use tower_cookies::{Cookie, CookieManagerLayer};

   use crate::{gate::AUTH_COOKIE_NAME, request_context::RequestContext};
   use super::super::{mw_implant_request_context, mw_require_request_context};


   #[tokio::test]
   async fn implement_request_context_middlewares_ok() {

      //***********************************************
      // experiment for the request context middlewares
      //***********************************************
     
      // #[derive(Clone, Debug)]
      // struct RequestContextPararell {
      //    user_id: i64
      // }

      // // error

      // type RCResult<T> = core::result::Result<T, RCError>;

      // #[derive(Debug)]
      // enum RCError{
      //    RequestContextNotInExtensions
      // }

      // impl IntoResponse for RCError {
      //    fn into_response(self) -> Response {
      //       // magic is to happen in map response middleware ServerError -> ClientError 
      
      //       let mut response = StatusCode::NOT_FOUND.into_response();
      //       response.extensions_mut().insert(self.to_string());
      
      //       response
      //    }
      // }

      // impl core::fmt::Display for RCError {
      //    fn fmt(
      //       &self,
      //       fmt: &mut core::fmt::Formatter,
      //    ) -> core::result::Result<(), core::fmt::Error> {
      //       write!(fmt, "{self:?}")
      //    }
      // }
      
      // impl std::error::Error for RCError {}


      // // from request parts
     
      // impl<S> FromRequestParts<S> for RequestContextPararell
      // where S: Send + Sync 
      // {
      //    type Rejection = RCError;

      //    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
      //       let request_context_from_req = parts
      //       .extensions
      //       .get::<RequestContextPararell>()
      //       .ok_or(RCError::RequestContextNotInExtensions)?
      //       .clone();

      //       Ok(request_context_from_req)
      //    }
      // }

      // // middlewares modifying the request

      // async fn mw_insert_request_context(mut req: Request<Body>, next: Next) -> impl IntoResponse {
      //    let request_context = RequestContextPararell { user_id: 1 };


      //    println!("mw_insert_request_context");

      //    req.extensions_mut().insert(request_context);

      //    next.run(req).await
      // }

      async fn remove_request_context(mut req: Request<Body>, next: Next) -> impl IntoResponse {

         println!("remove_request_context");

         req.extensions_mut().remove::<RequestContext>();

         next.run(req).await
      }

      // async fn mw_require_request_context(
      //    request_context: RCResult<RequestContextPararell>,
      //    req: Request<Body>,
      //    next: Next
      // ) -> RCResult<Response> {
         
      //    println!("mw_require_request_context");

      //    request_context?;

      //    Ok(next.run(req).await)
      // }

      // handler  

      async fn return_request_context(request_context: RequestContext) -> impl IntoResponse {
         println!("\nREQUEST CONTEXT: {}\n", request_context.user_id);
         request_context.user_id.to_string()
      }

      // routes

      let self_defeating_route = Router::new()
      .route("/request_context", get(return_request_context))
      .route_layer(middleware::from_fn(mw_require_request_context))
      .route_layer(middleware::from_fn(remove_request_context));

      let request_context_app = Router::new()
      .route("/request_context", get(return_request_context))
      .nest("/self_defeating", self_defeating_route)
      .layer(middleware::from_fn(mw_implant_request_context))
      .layer(CookieManagerLayer::new());

      let auth_token = format!("{}=1", AUTH_COOKIE_NAME); //Cookie::new(AUTH_COOKIE_NAME, "1");

      let request_context_response = request_context_app.clone().oneshot(
         Request::builder()
         .uri("/request_context")
         .header("Cookie", &auth_token) // the header the tower cooke middleware looks fore is Cookie
         .body(Body::empty())
         .unwrap()
      ).await.unwrap();

      let failed_response = request_context_app.clone().oneshot(
         Request::builder()
         .uri("/self_defeating/request_context")
         .header("Cookie", auth_token)
         .body(Body::empty())
         .unwrap()
      ).await.unwrap();

      let response_without_cookie = request_context_app.oneshot(   
         Request::builder()         
         .uri("/request_context")
         .body(Body::empty())
         .unwrap()
      ).await.unwrap();

      // I have now incorporated cookies into the middleare however I dont want to fail the request if no cookie is found
      // I guess that stuff can be done when I add some user authentication logic

      assert_eq!(request_context_response.status(), StatusCode::OK);
      assert_eq!(failed_response.status(), StatusCode::NOT_FOUND);
      assert_eq!(response_without_cookie.status(), StatusCode::BAD_REQUEST);


   }

   #[tokio::test]
   async fn middleware_experiment() {      

      #[derive(Clone, Debug)]
      struct MessageHolder(String);

      // ebables extraction of type from axum request
      
      impl<S> FromRequestParts<S> for MessageHolder
      where S: Send + Sync 
      {
         type Rejection = (StatusCode, &'static str);

         async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            let message_holder_from_req = parts
            .extensions
            .get::<MessageHolder>()
            .ok_or((StatusCode::NOT_FOUND, "message not found"))?
            .clone();

            Ok(message_holder_from_req)
         }
      }

      // middlewares modifying the request

      async fn mw_insert_message(mut req: Request<Body>, next: Next) -> impl IntoResponse {
         let message = MessageHolder(String::from("message"));

         req.extensions_mut().insert(message);

         next.run(req).await
      }

      async fn mw_approve_message(
         message_holder: MessageHolder, 
         mut req: Request<Body>, 
         next: Next
      ) -> impl IntoResponse {

         let approved_message = message_holder.0 + ": approved";

         // inserting it again will not change it, if you try to get a message holder then the first inserted will be returned
         // you have to get the mut and change it.
         if let Some(message) = req.extensions_mut().get_mut::<MessageHolder>() {
            message.0 = approved_message;
         }

         next.run(req).await
      }

      // handler

      async fn return_message(message: MessageHolder) -> impl IntoResponse {

         println!("\nMESSAGE: {}\n", message.0);
         assert!(message.0 == "message" || message.0 == "message: approved");
         
         message.0
      }

      // routes

      let approved_routes = Router::new()
      .route("/", get(return_message))
      .route_layer(middleware::from_fn(mw_approve_message));

      let test_app = Router::new()
      .route("/", get(return_message))
      .nest("/approved", approved_routes)
      .layer(middleware::from_fn(mw_insert_message));

      let response1 = test_app.clone().oneshot(
         Request::builder()
         .uri("/")
         .body(Body::empty())
         .unwrap()
      ).await.unwrap();

      let response2 = test_app.oneshot(
         Request::builder()
         .uri("/approved")
         .body(Body::empty())
         .unwrap()
      ).await.unwrap();

      assert_eq!(response1.status(), StatusCode::OK);
      assert_eq!(response2.status(), StatusCode::OK);
      
   }
}