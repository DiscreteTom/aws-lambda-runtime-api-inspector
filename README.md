# AWS Lambda Runtime API Inspector

[![version](https://img.shields.io/github/v/tag/DiscreteTom/aws-lambda-runtime-api-inspector?label=release&style=flat-square)](https://github.com/DiscreteTom/aws-lambda-runtime-api-inspector/releases/latest)
![license](https://img.shields.io/github/license/DiscreteTom/aws-lambda-runtime-api-inspector?style=flat-square)
![rust](https://img.shields.io/badge/built_with-rust-DEA584?style=flat-square)

A lambda layer to print the runtime API requests and responses.

## Usage

### As a Lambda Layer

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-runtime-api-inspector/releases/latest). You can also build it yourself by running `cargo build --release`, then zip `scripts/entry.sh` with `target/release/aws-lambda-runtime-api-inspector`.
2. Upload the zip as a lambda layer. Add the layer to your lambda function.
3. Add an environment variable `AWS_LAMBDA_EXEC_WRAPPER` to the lambda function with the value `/opt/entry.sh` to enable the runner.
4. Configure the [environment variables](#environment-variables) below to set the command you want to run.

### As a Binary Executable

If you are using a custom lambda runtime (for rust, golang, c++, etc) or container image, you can run the filter as a parent process of your main handler process.

1. Download the prebuilt zip from the [release page](https://github.com/DiscreteTom/aws-lambda-runtime-api-inspector/releases/latest) to get the `aws-lambda-runtime-api-inspector` executable. You can also build it yourself by running `cargo build --release`.
2. Modify the entry command of the lambda function to `aws-lambda-runtime-api-inspector <handler-command> <handler-args>`
3. Configure the [environment variables](#environment-variables) below to set the command you want to run.

### Environment Variables

- `AWS_LAMBDA_RUNTIME_API_INSPECTOR_TARGETS`
  - Specify which requests and responses to print.
  - Valid values:
    - `NextInvocationRequest`
    - `NextInvocationResponse`
    - `NextInvocation`, equals to `NextInvocationRequest,NextInvocationResponse`
    - `InvocationResponseRequest`
    - `InvocationResponseResponse`
    - `InvocationResponse`, equals to `InvocationResponseRequest,InvocationResponseResponse`
    - `InitializationErrorRequest`
    - `InitializationErrorResponse`
    - `InitializationError`, equals to `InitializationErrorRequest,InitializationErrorResponse`
    - `InvocationErrorRequest`
    - `InvocationErrorResponse`
    - `InvocationError`, equals to `InvocationErrorRequest,InvocationErrorResponse`
    - Multiple values can be separated by `,`.
  - If not set, all Lambda runtime API requests and responses will be printed.
  - For more information about the runtime API, see [AWS Lambda Runtime API](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html).

## Example Output

Here is an example output of one cold start invocation, with all requests and responses printed:

```
Runtime API Request: GET /2018-06-01/runtime/invocation/next headers: {"host": "127.0.0.1:3000", "accept": "*/*", "user-agent": "AWS_Lambda_Cpp/0.2.6"} body: b""
Runtime API Response: [200 OK] headers: {"content-type": "application/json", "lambda-runtime-aws-request-id": "04feef66-1b37-4f4f-85e5-c66ff4b5a9c0", "lambda-runtime-deadline-ms": "1713968525978", "lambda-runtime-invoked-function-arn": "arn:aws:lambda:us-east-1:123123123123:function:test", "lambda-runtime-trace-id": "Root=1-6629156f-0a9d5a7f74a36b1059e065fe;Parent=7c4d733f273bdab1;Sampled=0;Lineage=a3f0941e:0", "date": "Wed, 24 Apr 2024 14:21:35 GMT", "content-length": "49"} body: b"{\"key1\":\"value1\",\"key2\":\"value2\",\"key3\":\"value3\"}"
Runtime API Request: POST /2018-06-01/runtime/invocation/04feef66-1b37-4f4f-85e5-c66ff4b5a9c0/response headers: {"host": "127.0.0.1:3000", "accept": "*/*", "content-type": "application/json", "user-agent": "AWS_Lambda_Cpp/0.2.6", "content-length": "33"} body: b"{\"statusCode\":200,\"body\":\"hello\"}"
Runtime API Response: [202 Accepted] headers: {"content-type": "application/json", "date": "Wed, 24 Apr 2024 14:21:35 GMT", "content-length": "16"} body: b"{\"status\":\"OK\"}\n"
Runtime API Request: GET /2018-06-01/runtime/invocation/next headers: {"host": "127.0.0.1:3000", "accept": "*/*", "user-agent": "AWS_Lambda_Cpp/0.2.6"} body: b""
```

## FAQ

- Q: How does this work?
  - By using [AWS Lambda Runtime Proxy](https://github.com/DiscreteTom/aws-lambda-runtime-proxy) this tool can intercept the lambda handler process's runtime API requests.

## [CHANGELOG](./CHANGELOG.md)
