#![allow(unused_imports)]

#[cfg(test)]
mod tests {
   use axum::{extract::Request, http::request::Parts};
   use reqwest::RequestBuilder;


   #[tokio::test]
   async fn middleware_experiment() {

      use axum::{
         body::Body,
         extract::FromRequestParts,
         http::{Request, StatusCode},
         middleware::{self, Next},
         response::IntoResponse,
         routing::get,
         Extension,
         Router
      };
      use tower::ServiceBuilder;
      use tower::ServiceExt;

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