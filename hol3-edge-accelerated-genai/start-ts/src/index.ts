import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import type { Context, Next } from 'hono';
import { loadConfig, Config } from './config';
import { buildChatRequestPayload } from './ollama';
import { Md5 } from 'ts-md5';
let app = new Hono();

interface SentimentAnalysisRequest {
  text: string;
}

interface SentimentAnalysisResponse {
  mood: string;
}

app.post('/sentiment-analysis', async (c: Context) => {
  const json = await c.req.json();
  if (!json || typeof json.text !== 'string') {
    return c.json({ error: 'Invalid payload: expected { text: string }' }, 400);
  }
  const model = json as SentimentAnalysisRequest;

  const config: Config | undefined = loadConfig();
  if (!config) {
    return c.json({ error: "Invalid configuration" }, 500);
  }

  // Hands-On-Labs 3 - Edge Accelerated Generative AI
  // TASK 1: Implement Sentiment Analysis using Ollama API
  //          use fetch API for creating and sending a new Outbound HTTP request
  //          to perform a sentiment analysis. 
  //          Grab the mood from the response received by Ollama
  //          In case of any error, return a 500 
  //          Desired Ollama API endpoint: /api/chat API
  //          Request Payload can be constructed using `buildChatRequestPayload` defined in src/ollama.ts
  //          Ollama Docs: https://docs.ollama.com/api/chat
  //          Spin SDK Docs: https://spinframework.dev/v3/http-outbound
  // TASK 2: Accelerate Sentiment Analysis using Distributed Key Value Store
  //          Sanitize the input text by removing spaces, punctuation, and converting to lowercase
  //          Use the Md5 object (in scope) to compute a hash of the sanitized input text
  //          Use Spin SDK Key Value Store to check if a cached response exists for the computed hash
  //          If a cached response exists, return it directly
  //          If no cached response exists, proceed to call the Ollama API
  //          After receiving a successful response from the Ollama API, cache the response
  //          in the Key Value Store using the computed hash as the key

  // placeholder for mood from Ollama response
  const payload = {
    mood: "neutral" // replace with actual mood
  } as SentimentAnalysisResponse;

  return c.json(payload);
});

fire(app);

