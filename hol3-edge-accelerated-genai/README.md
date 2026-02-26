# Hands-On-Labs 3: Edge Accelerated Generative AI

## Cloning The Repository and Setting Up The Developer Machine

Clone this repository ([akamai-developers/serverless-wasm-functions-workshop](https://github.com/akamai-developers/serverless-wasm-functions-workshop)) and navigate to your local copy using the terminal.

Ensure you've configured your developer machine as explained in the [Setup Instructions](/developer-machine-setup/).

## Edge Accelerated Generative AI

Large Language Models (LLMs) unlock a new set of capabilities that we as developers could either integrate into existing applications, or build new products and services around. You could take those capabilities to a whole new level, by combining generative AI with zero-cold-start times and ultra-low latency provided by WebAssembly and *Akamai Functions*.

This Hands-On-Labs exercise is all about building your own Sentiment Analysis. You'll start by simply integrating the LLM into the Spin application using its HTTP API. Finally, we will accelerate the application by caching responses created by the LLM using the distributed key value store provided by *Akamai Functions*.

## Starting Points

You can find *starting points* for [TypeScript in `start-ts`](./start-ts/) and for [Rust in `start-rs`](./start-rust/). It's a Spin application that exposes an HTTP API at `/api/`. The component responding to incoming requests at `/api/...` and has a few simple handlers registered using either the built-in HTTP router (in case of Rust) or the [Hono](https://hono.dev/) router in case of TypeScript.

Without further ado, you can compile and run the *starting point* using the `spin build` and `spin up` commands (or you combine both commands by executing `spin up --build`).

### Task 1: Sentiment Analysis using an LLM

To implement sentiment analysis, create an outbound HTTP `POST` request to the Ollama `/api/chat` endpoint using the Spin SDK’s HTTP client ([Sending HTTP requests in Spin](https://spinframework.dev/v3/http-outbound)).

Construct a request payload containing the target model, the text for analysis, and a system prompt to define the sentiment task ([Ollama API Specs](https://docs.ollama.com/api/chat)).

Once the response is received, deserialize the JSON body into a structured object to extract the model's message content. Ensure you handle any connectivity issues or parsing failures by returning a `500 Internal Server Error`.

*HINT: verify that the stream parameter is disabled to simplify the response handling within your application logic.*

#### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Akamai Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.

### Task 2: Caching LLM responses using the key value store

To accelerate sentiment analysis, you can implement a caching layer using the fully managed Key Value store ([Key Value Store docs](https://spinframework.dev/v3/kv-store-api-guide)) to avoid redundant API calls.

First, sanitize the input text by removing whitespace and punctuation and converting it to lowercase, to generate a unique key. (The starting point for this lab is setup with MD5 libraries in place).

Before calling the Ollama API, query the key value store with this generated key for the incoming request to check for an existing sentiment result. If a value is found in key value store, return it immediately; otherwise, proceed with the API request as previously instructed.

Once a successful response is received from Ollama, store the result in the key value store using the computed hash as the key to accelerate all future identical requests.

#### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Akamai Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.
