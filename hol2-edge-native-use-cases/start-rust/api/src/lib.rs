use anyhow::Result;
use serde::Deserialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::http_component;

#[http_component]
fn entrypoint(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/api/greet/:name", greet);
    router.get("/api/ping", ping);
    // Hands-On-Labs 2 Edge Native Use Cases
    // TASK 1: Add another handler and respond to incoming POST requests at /api/add
    Ok(router.handle(req))
}

// TASK 1: The add handler should grab values from the request payload
//         The corresponding struct `Payload` is located at the end of this file
//         The popular crates serde and serde_json are already configured as dependency
//         For valid requests, add both numbers and return the sum as Response
// Hint: The method name starts with an _ (underscore) to ensure Rust compiler does not yell at you
//       Once you've hooked up the method using the Router, remove the leading underscore
fn _add(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    // Hint: Use generic serde_json::from_slice with Payload to deserialize the req.body()
    //       If from_slice returns an Err, send a 400 bad request
    //       If you decide to create a dedicated struct for the response,
    //       consider implementing the IntoBody trait from the spin_sdk::http::conversions module
    Ok(Response::new(500, "Not Implemented"))
}

fn greet(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(name) = params.get("name") else {
        return Ok(Response::new(400, "Bad Request"));
    };

    // TASK 2: Instead of hard-coding "Hello", load hello from an variable
    // Hint: Variables must be defined in Spin Manifest
    //       Variables are defined on the scope of the application
    //       Individual components must be granted access to variables
    //       https://spinframework.dev/v3/variables#application-variables
    // Hint: The spin_sdk crate comes with batteries included
    //       Check spin_sdk::variables
    let message = format!("Hello, {name}!");
    Ok(ResponseBuilder::new(200)
        .header("content-type", "text/plain")
        .body(message).build())
}

fn ping(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    // TASK3: Use Key-Value store to track how many invocations hit this endpoint
    //          Sent a custom x-counter header along the actual value of the invocation counter
    // Hint: Spin provides an API for you to persist data in a key value store managed by Spin. 
    //       This key value store allows Spin developers to persist non-relational data 
    //       across application invocations.
    //       https://spinframework.dev/v3/kv-store-api-guide
    // Hint: The spin_sdk crate comes with batteries included
    //       Check spin_sdk::key_value and the Store struct to get started
    Ok(ResponseBuilder::new(200)
    .header("content-type", "text/plain")
    .body("pong")
    .build())
}

#[derive(Deserialize)]
pub struct Payload {
    #[serde(rename="operandA")]
    pub operand_a: f32,
    #[serde(rename="operandB")]
    pub operand_b: f32
}

impl Payload {
    pub fn sum(&self) -> f32 {
        self.operand_a + self.operand_b
    }
}