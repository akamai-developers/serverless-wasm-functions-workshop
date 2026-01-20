use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Params, Request, RequestBuilder, Response, ResponseBuilder, Router, send};
use spin_sdk::key_value::Store;
use spin_sdk::{http_component};

use crate::config::Config;
use crate::ollama::{ChatRequest, ChatResponse};

mod config;
mod ollama;

#[http_component]
fn handle_edge_accelerated_genai(req: Request) -> anyhow::Result<impl IntoResponse> {
   let mut router = Router::default();
   router.post_async("/sentiment-analysis", perform_sentiment_analysis);
   Ok(router.handle(req))
}

async fn perform_sentiment_analysis(req: Request, _p: Params) -> Result<impl IntoResponse>{
    let Ok(_config) = Config::try_load() else {
        return Ok(Response::new(500, "Configuration invalid!"));
    };
    let Ok(model) = serde_json::from_slice::<SentimentAnalysisRequestModel>(req.body()) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    let sanitized = sanitize(model.text.clone());
    let hash: String = format!("{:x}", md5::compute(sanitized.clone()));
    let store = Store::open_default()?;
    match store.get_json::<SentimentAnalysisResponseModel>(hash.clone())? {
        Some(cached_response) => {
            println!("Cache hit for: '{}'", sanitized);
            return Ok(ResponseBuilder::new(200)
                .header("Content-Type", "application/json")
                .body(cached_response)
                .build());
        },
        None => {
            println!("Cache miss for: '{}'", sanitized);
        }
    }
    // Hands-On-Labs 3 - Edge Accelerated Generative AI
    // TASK 1: Implement Sentiment Analysis using Ollama API
    //          use RequestBuilder and Outbound HTTP provided by Spin SDK
    //          to perform a sentiment analysis. 
    //          Grab the response and deserialize it into OllamaGenerateResponseModel
    //          In case of any error, return a 500 
    //          Desired Ollama API endpoint: /api/generate API
    //          Request Payload can be constructed using OllamaGenerateRequestModel
    //          Ollama Docs: https://docs.ollama.com/api/generate
    //          Spin SDK Docs: https://spinframework.dev/v3/http-outbound
    // TASK 2: Accelerate Sentiment Analysis using Distributed Key Value Store
    //          Sanitize the input text by removing spaces, punctuation, and converting to lowercase
    //          Use the md5 crate to compute a hash of the sanitized input text
    //          Use Spin SDK Key Value Store to check if a cached response exists for the computed hash
    //          If a cached response exists, return it directly
    //          If no cached response exists, proceed to call the Ollama API
    //          After receiving a successful response from the Ollama API, cache the response
    //          in the Key Value Store using the computed hash as the key


    let ollama_endpoint = format!("{}/api/chat", _config.ollama_api_url);
    let ollama_request_payload = ChatRequest::new(
        model.text.clone(),
        _config.model.clone(), 
        _config.temperature
    );
    
    let ollama_req = RequestBuilder::new(spin_sdk::http::Method::Post,
        ollama_endpoint)
        .header("Content-Type", "application/json")
        .body(ollama_request_payload).build();
    
    let ollama_res: Response = send(ollama_req).await?;
    if ollama_res.status() != &200 {
        println!("Error from Ollama API: {}", ollama_res.status());
        return Ok(Response::new(500, "Error from Ollama API"));
    }
    
    let ollama_response_payload = match serde_json::from_slice::<ChatResponse>(ollama_res.body())
    {
        Ok(res) => res, 
        Err(e) => {
            println!("Error deserializing Ollama response: {}", e);
            return Ok(Response::new(500, "Error deserializing Ollama response"));
        }
    };
    let payload = SentimentAnalysisResponseModel {
        mood: ollama_response_payload.message.content.trim().to_lowercase()
    };
    if store.set_json(hash, &payload).is_err() {
        println!("Error caching response");
    }
    // grab sentiment from the response
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}

#[derive(Deserialize)]
pub struct SentimentAnalysisRequestModel {
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct SentimentAnalysisResponseModel {
    pub mood: String,
}

impl IntoBody for SentimentAnalysisResponseModel{
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self)
            .with_context(|| "Error while serializing response model")
            .unwrap()
    }
}

 
 fn sanitize(message: String) -> String {
    message.replace("\n", " ")
    .replace("\r", " ")
    .replace(".", " ")
        .replace(",", " ")
        .replace("?", " ")
        .replace("!", " ")
        .replace("\t", " ")
        .replace(" ", "")
        .trim()
        .to_lowercase()
        .to_string()
    }
 