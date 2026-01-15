// For Hono documentation refer to https://hono.dev/docs/
import { Hono } from 'hono';
import { HTTPException } from 'hono/http-exception'
import { fire } from 'hono/service-worker';
import type { Context, Next } from 'hono';
import { logger } from 'hono/logger';
import * as variables from '@spinframework/spin-variables';
import * as kv from '@spinframework/spin-kv';
let app = new Hono();
const decoder = new TextDecoder();

app.use(logger());
// HOL 2.1: The add handler should grab values from the request payload
//          The corresponding struct `Payload` is located at the end of this file
//          For valid requests, add both numbers and return the sum as response
const add = async (c: Context) => {
  let payload: Payload;
  try {
    payload = await c.req.json();
  } catch (e) {
    c.status(400);
    return c.text("Bad Request");
  }
  const sum = payload.operandA + payload.operandB;
  return c.json({ result: sum });
};

app.post('/api/add', add);

app.get('/api/greet/:name', (c: Context) => {
  const greeting = variables.get("greeting") || "Hello";
  const name = c.req.param('name');
  return c.text(`${greeting}, ${name}!`);
});

app.get('/api/ping', (c: Context) => {
  const store = kv.openDefault();
  let counter = 0;
  if (store.exists('counter')) {
    counter = parseInt(decoder.decode(store.get('counter')!), 10);
  }
  counter += 1;
  store.set('counter', counter.toString());
  c.res.headers.set('x-counter', counter.toString());
  return c.text('pong')
});

interface Payload {
  operandA: number;
  operandB: number;
}

fire(app);

