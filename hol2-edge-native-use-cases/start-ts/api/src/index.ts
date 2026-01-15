// For Hono documentation refer to https://hono.dev/docs/
import { Hono } from 'hono';
import { HTTPException } from 'hono/http-exception'
import { fire } from 'hono/service-worker';
import type { Context, Next } from 'hono';
import { logger } from 'hono/logger';

let app = new Hono();

app.use(logger());
// HOL 2.1: Add another handler and respond to incoming POST requests at /add
//          See the `add` function at the end of this file

app.get('/api/greet/:name', (c: Context) => {
   // HOL 2.2: Instead of hard-coding "Hello", load hello from an variable
    // Hint: Variables must be defined in Spin Manifest
    //       Variables are defined on the scope of the application
    //       Individual components must be granted access to variables
    //       https://spinframework.dev/v3/variables#application-variables
    // Hint: Spin SDK for TS/JS is modular check @spinframework/* for spin-variables
    //       https://www.npmjs.com/search?q=%40spinframework
    //       and https://www.npmjs.com/package/@spinframework/spin-variables
  return c.text(`Hello, ${c.req.param('name')}!`);
});

app.get('/api/ping', (c: Context) => {
   // HOL 2.3: Use Key-Value store to track how many invocations hit this endpoint
    //          Sent a custom X-Count header along the actual value of the invocation counter
    // Hint: Spin provides an API for you to persist data in a key value store managed by Spin. 
    //       This key value store allows Spin developers to persist non-relational data 
    //       across application invocations.
    //       https://spinframework.dev/v3/kv-store-api-guide
    // Hint: Spin SDK for TS/JS is modular check @spinframework/* for spin-variables
    //       https://www.npmjs.com/search?q=%40spinframework
    //       and https://www.npmjs.com/package/@spinframework/spin-kv
  return c.text('pong')
});

// HOL 2.1: The add handler should grab values from the request payload
//          The corresponding struct `Payload` is located at the end of this file
//          For valid requests, add both numbers and return the sum as response
const add = async (c: Context) => {
  // Hint: use await c.req.json() to access request payload
  //       see https://hono.dev/docs/api/request#json
  throw new HTTPException(500, { message: "Not Implemented"})
};

fire(app);

