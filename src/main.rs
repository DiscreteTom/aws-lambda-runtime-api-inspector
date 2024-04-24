use aws_lambda_runtime_proxy::{LambdaRuntimeApiClient, Proxy};

// targets bit flags
const NEXT_INVOCATION: usize = 1 << 0;
const INVOCATION_RESPONSE: usize = 1 << 1;
const INITIALIZATION_ERROR: usize = 1 << 2;
const INVOCATION_ERROR: usize = 1 << 3;

#[tokio::main]
async fn main() {
  let targets = std::env::var("AWS_LAMBDA_RUNTIME_API_INSPECTOR_TARGETS")
    .map(|s| {
      s.split(',')
        .map(|s| match s {
          "NextInvocation" => NEXT_INVOCATION,
          "InvocationResponse" => INVOCATION_RESPONSE,
          "InitializationError" => INITIALIZATION_ERROR,
          "InvocationError" => INVOCATION_ERROR,
          _ => panic!("Invalid target for aws-lambda-runtime-api-inspector: {}", s),
        })
        .fold(0, |acc, x| acc | x)
    })
    .unwrap_or(NEXT_INVOCATION | INVOCATION_RESPONSE | INITIALIZATION_ERROR | INVOCATION_ERROR);

  Proxy::default()
    .spawn()
    .await
    .server
    .serve(move |req| async move {
      let path = req.uri().path();

      if (targets & NEXT_INVOCATION != 0 && path == "/2018-06-01/runtime/invocation/next")
        || (targets & INVOCATION_RESPONSE != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/response"))
        || (targets & INITIALIZATION_ERROR != 0 && path == "/2018-06-01/runtime/init/error")
        || (targets & INVOCATION_ERROR != 0
          && path.starts_with("/2018-06-01/runtime/invocation/")
          && path.ends_with("/error"))
      {
        println!("{:?}", req);
        let res = LambdaRuntimeApiClient::forward(req).await;
        if let Ok(res) = &res {
          println!("{:?}", res);
        }
        res
      } else {
        // just forward the request
        LambdaRuntimeApiClient::forward(req).await
      }
    })
    .await
}
