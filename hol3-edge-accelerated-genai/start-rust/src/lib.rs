use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::{http_component};

use crate::config::Config;

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
    let Ok(_model) = serde_json::from_slice::<SentimentAnalysisRequestModel>(req.body()) else {
        return Ok(Response::new(400, "Bad Request"));
    };
    
    // Hands-On-Labs 3 - Edge Accelerated Generative AI
    // TASK 1: Implement Sentiment Analysis using Ollama API
    //          use RequestBuilder and Outbound HTTP provided by Spin SDK
    //          to perform a sentiment analysis. 
    //          Grab the response and deserialize it into OllamaGenerateResponseModel
    //          In case of any error, return a 500 
    //          Desired Ollama API endpoint: /api/chat API
    //          Request Payload can be constructed using OllamaGenerateRequestModel
    //          Ollama Docs: https://docs.ollama.com/api/chat
    //          Spin SDK Docs: https://spinframework.dev/v3/http-outbound
    // TASK 2: Accelerate Sentiment Analysis using Distributed Key Value Store
    //          Sanitize the input text by removing spaces, punctuation, and converting to lowercase
    //          Use the md5 crate to compute a hash of the sanitized input text
    //          Use Spin SDK Key Value Store to check if a cached response exists for the computed hash
    //          If a cached response exists, return it directly
    //          If no cached response exists, proceed to call the Ollama API
    //          After receiving a successful response from the Ollama API, cache the response
    //          in the Key Value Store using the computed hash as the key
    
    // placeholder for mood from Ollama response
    let payload = SentimentAnalysisResponseModel {
        mood: String::from("neutral"), // replace with actual mood
    };
    
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}


#[derive(Deserialize)]
pub struct SentimentAnalysisRequestModel {
    pub text: String,
}

#[derive(Serialize)]
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
