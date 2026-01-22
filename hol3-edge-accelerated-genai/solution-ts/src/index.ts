import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import type { Context } from 'hono';
import { loadConfig, Config } from './config';
import { buildChatRequestPayload } from './ollama';
import { Md5 } from 'ts-md5';
import * as kv from "@spinframework/spin-kv";

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

  const store = kv.openDefault();
  const sanitizedText = model.text.replace(/\s+/g, '').replace(/[^\w]/g, '').toLowerCase();
  const hash = Md5.hashStr(sanitizedText) as string;
  if (store.exists(hash)) {
    const cachedResponse = store.getJson(hash) as SentimentAnalysisResponse | null;
    if (cachedResponse) {
      console.log(`Cache hit for "${model.text}"`);
      return c.json(cachedResponse);
    }
  }

  const ollamaPayload = buildChatRequestPayload(model.text, config.model, config.temperature);
  const ollamaReq = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json',
    },
    body: JSON.stringify(ollamaPayload),
  } as RequestInit;

  const ollamaRes = await fetch(`${config.ollamaUrl}/api/chat`, ollamaReq);
  if (!ollamaRes.ok) {
    return c.json({ error: "Error from Ollama API" }, 500);
  }
  const ollamaData = await ollamaRes.json();
  if (!ollamaData || !ollamaData.message || !ollamaData.message.content) {
    return c.json({ error: "Invalid response from Ollama API" }, 500);
  }
  const payload = {
    mood: ollamaData.message.content
  } as SentimentAnalysisResponse;

  try {
    await store.setJson(hash, payload)
  } catch (e) {
    console.log(`Error caching response for "${model.text}": ${e}`);
  }
  return c.json(payload);
});

fire(app);

