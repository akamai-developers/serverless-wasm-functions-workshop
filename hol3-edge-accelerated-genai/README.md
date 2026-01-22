# Hands-On-Labs 3: Edge Accelerated Generative AI

## Cloning The Repository and Setting Up The Developer Machine

Clone this repository ([akamai-developers/serverless-wasm-functions-workshop](https://github.com/akamai-developers/serverless-wasm-functions-workshop)) and navigate to your local copy using the terminal.

Ensure you've configured your developer machine as explained in the [Setup Instructions](/developer-machine-setup/).

## Edge Accelerated Generative AI

Large Language Models (LLMs) unlock a new set of capabilities that we as developers could either integrate into existing applications, or build new products and services around. You could take those capabilities to a whole new level, by combining generative AI with zero-cold-start times and ultra-low latency provided by WebAssembly and *Serverless Wasm Functions*.

This Hands-On-Labs exercise is all about building your own Sentiment Analysis. You'll start by simply integrating the LLM into the Spin application using its HTTP API. Finally, we will accelerate the application by caching responses created by the LLM using the distributed key value store provided by *Serverless Wasm Functions*.

## Starting Points

You can find *starting points* for [TypeScript in `start-ts`](./start-ts/) and for [Rust in `start-rs`](./start-rust/). It's a Spin application that exposes an HTTP API at `/api/`. The component responding to incoming requests at `/api/...` and has a few simple handlers registered using either the built-in HTTP router (in case of Rust) or the [Hono](https://hono.dev/) router in case of TypeScript.

Without further ado, you can compile and run the *starting point* using the `spin build` and `spin up` commands (or you combine both commands by executing `spin up --build`).

### Task 1: Sentiment Analysis using an LLM

TBD

#### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.

### Task 2: Caching LLM responses using key value store

TBD

#### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.