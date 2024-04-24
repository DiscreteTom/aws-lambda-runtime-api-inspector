use aws_lambda_runtime_proxy::{LambdaRuntimeApiClient, Proxy};
use http::{Request, Response};
use http_body_util::{BodyExt, Full};

// targets bit flags
const NEXT_INVOCATION_REQUEST: usize = 1 << 0;
const NEXT_INVOCATION_RESPONSE: usize = 1 << 1;
const INVOCATION_RESPONSE_REQUEST: usize = 1 << 2;
const INVOCATION_RESPONSE_RESPONSE: usize = 1 << 3;
const INITIALIZATION_ERROR_REQUEST: usize = 1 << 4;
const INITIALIZATION_ERROR_RESPONSE: usize = 1 << 5;
const INVOCATION_ERROR_REQUEST: usize = 1 << 6;
const INVOCATION_ERROR_RESPONSE: usize = 1 << 7;

#[tokio::main]
async fn main() {
  let targets = std::env::var("AWS_LAMBDA_RUNTIME_API_INSPECTOR_TARGETS")
    .map(|s| {
      s.split(',')
        .map(|s| match s {
          "NextInvocationRequest" => NEXT_INVOCATION_REQUEST,
          "NextInvocationResponse" => NEXT_INVOCATION_RESPONSE,
          "NextInvocation" => NEXT_INVOCATION_REQUEST | NEXT_INVOCATION_RESPONSE,
          "InvocationResponseRequest" => INVOCATION_RESPONSE_REQUEST,
          "InvocationResponseResponse" => INVOCATION_RESPONSE_RESPONSE,
          "InvocationResponse" => INVOCATION_RESPONSE_REQUEST | INVOCATION_RESPONSE_RESPONSE,
          "InitializationErrorRequest" => INITIALIZATION_ERROR_REQUEST,
          "InitializationErrorResponse" => INITIALIZATION_ERROR_RESPONSE,
          "InitializationError" => INITIALIZATION_ERROR_REQUEST | INITIALIZATION_ERROR_RESPONSE,
          "InvocationErrorRequest" => INVOCATION_ERROR_REQUEST,
          "InvocationErrorResponse" => INVOCATION_ERROR_RESPONSE,
          "InvocationError" => INVOCATION_ERROR_REQUEST | INVOCATION_ERROR_RESPONSE,
          _ => panic!("Invalid target for aws-lambda-runtime-api-inspector: {}", s),
        })
        .fold(0, |acc, x| acc | x)
    })
    // default to all targets
    .unwrap_or(usize::MAX);

  Proxy::default()
    .spawn()
    .await
    .server
    .serve(move |req| async move {
      let path = req.uri().path();

      let need_print_request = (targets & NEXT_INVOCATION_REQUEST != 0
        && path == "/2018-06-01/runtime/invocation/next")
        || (targets & INVOCATION_RESPONSE_REQUEST != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/response"))
        || (targets & INITIALIZATION_ERROR_REQUEST != 0
          && path == "/2018-06-01/runtime/init/error")
        || (targets & INVOCATION_ERROR_REQUEST != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/error"));

      let need_print_response = (targets & NEXT_INVOCATION_RESPONSE != 0
        && path == "/2018-06-01/runtime/invocation/next")
        || (targets & INVOCATION_RESPONSE_RESPONSE != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/response"))
        || (targets & INITIALIZATION_ERROR_RESPONSE != 0
          && path == "/2018-06-01/runtime/init/error")
        || (targets & INVOCATION_ERROR_RESPONSE != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/error"));

      let res = if need_print_request {
        // collect request and print it
        let (parts, body) = req.into_parts();
        let bytes = body.collect().await.unwrap().to_bytes();
        println!(
          "Runtime API Request: {} {} headers: {:?} body: {:?}",
          parts.method, parts.uri, parts.headers, bytes
        );

        // forward the request, collect the response
        LambdaRuntimeApiClient::new()
          .await
          .send_request(Request::from_parts(parts, Full::new(bytes)))
          .await
          .unwrap()
      } else {
        // no need to collect request, just forward it
        LambdaRuntimeApiClient::new()
          .await
          .send_request(req)
          .await
          .unwrap()
      };

      // no matter we print the request or not, we need to collect the response
      // why? see https://github.com/DiscreteTom/aws-lambda-runtime-proxy/blob/ccf207c90e0d6bb4d01651b35b0d14c64256f4cd/src/client.rs#L69-L71
      let (parts, body) = res.into_parts();
      let bytes = body.collect().await.unwrap().to_bytes();

      // print the response
      if need_print_response {
        println!(
          "Runtime API Response: [{}] headers: {:?} body: {:?}",
          parts.status, parts.headers, bytes
        );
      }

      // return the response
      Ok(Response::from_parts(parts, Full::new(bytes)))
    })
    .await
}
