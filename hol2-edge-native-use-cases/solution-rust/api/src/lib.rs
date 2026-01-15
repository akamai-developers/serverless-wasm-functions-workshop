use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::key_value::Store;
use spin_sdk::{http_component, variables};


#[http_component]
fn entrypoint(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/api/greet/:name", greet);
    router.get("/api/ping", ping);
    router.post("/api/add", add);
    Ok(router.handle(req))
}

fn add(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let Ok(payload) = serde_json::from_slice::<Payload>(req.body()) else {
        return Ok(Response::new(400, "Bad Request"));
    };

    let model = SumModel {
        sum: payload.sum()
    };
    
    Ok(ResponseBuilder::new(200)
        .header("content-type", "application/json")
        .body(model)
        .build())
}

fn greet(_req: Request, params: Params) -> Result<impl IntoResponse> {
    let Some(name) = params.get("name") else {
        return Ok(Response::new(400, "Bad Request"));
    };

    let greeting = variables::get("greeting").unwrap_or("Hello".to_string());

    let message = format!("{greeting}, {name}!");
    Ok(ResponseBuilder::new(200)
        .header("content-type", "text/plain")
        .body(message).build())
}

fn ping(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let store = Store::open_default()?;
    
    let mut counter = match store.get("counter")? {
        Some(v) =>  {
            String::from_utf8(v)
                .ok()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0)
        },
        None => 0
    };

    counter += 1;
    store.set("counter", counter.to_string().as_bytes())?;
    Ok(ResponseBuilder::new(200)
        .header("x-counter", counter.to_string().as_str())
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

#[derive(Serialize)]
pub struct SumModel {
    pub sum: f32
}

impl IntoBody for SumModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("Error serializing response into JSON")
    }
}