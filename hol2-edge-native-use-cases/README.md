# Hands-On-Labs 2: Edge Native Use Cases

## Cloning The Repository and Setting Up The Developer Machine

Clone this repository ([akamai-developers/serverless-wasm-functions-workshop](https://github.com/akamai-developers/serverless-wasm-functions-workshop)) and navigate to your local copy using the terminal.

Ensure you've configured your developer machine as explained in the [Setup Instructions](/developer-machine-setup/).

## Edge Native Use Cases

Edge native applications come in many different shapes and address a vast variety of requirements. That said, we come of with four common scenarios that you'll tackle as part of this hands-on-labs exercise:

1. Implement HTTP-Routing in Spin applications
2. Specify and use application variables to configure Spin applications
3. Persisting data across multiple invocations using the distributed key-value store
4. Building multi-component Spin applications

## Starting Points

You can find *starting points* for [TypeScript in `start-ts`](./start-ts/) and for [Rust in `start-rs`](./start-rust/). It's a Spin application that exposes an HTTP API at `/api/`. The component responding to incoming requests at `/api/...` and has a few simple handlers registered using either the built-in HTTP router (in case of Rust) or the [Hono](https://hono.dev/) router in case of TypeScript.

Without further ado, you can compile and run the *starting point* using the `spin build` and `spin up` commands (or you combine both commands by executing `spin up --build`).

## Task 1: Implement HTTP-Routing in Spin applications

Spin applications have full access to the incoming HTTP request, allowing you to build real-world HTTP-based functions. Building serverless HTTP-APIs and running them on the edge is pretty common and accelerates frequent user interactions. Complete the following tasks to familiarize yourself with the routing capabilities within the bounds of a Spin application:

- Use the router API to handle incoming `POST` requests at `/api/add`, the *starting point* already contains a corresponding handler function (`add`) which you could wire up to the route registration.
- Take the incoming request payload and turn it into an instance of the `Payload` structure (or interface in the case of TypeScript) which exists already in the *starting point*
- Respond to requests with invalid or mal-formatted payloads with a HTTP `400` (`Bad Request`)
- Calculate the sum of both operands and return it as JSON (`Content-Type: application/json`)

### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.

## Task 2: Specify and Use Application Variables

Spin supports dynamic application variables. Instead of being static, your application could use variables, that could be updated without modifying the application source code, creating a simpler experience for rotating secrets, updating API endpoints, and more.

Variables are defined on the scope of the application in the Spin Manifest (`spin.toml`) using a `[variables]` block. Once you've defined a variable, you must grant your application component access to your variables.

To access a variable from source code, you can use the APIs provided by the language-specific Spin SDKs. Consult the ["Application Variables" section of the Spin documentation](https://spinframework.dev/v3/variables) and learn more about how to use them.

Application variables could be specified using the `--variable` flag when running your application locally with `spin up` or when deploying to *Serverless Wasm Functions* using `spin aka deploy`.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.

### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

## Task 3: Persisting Data

*Serverless Wasm Functions* provides access to a fully-managed key value store, allowing you to persist data across multiple function invocations. The fully-managed key value store is globally distributed and isolated per application.

To interact with the *Serverless Wasm Functions* key value store, you have to:

- Specify which components require access to the key value store in the application manifest (`spin.toml`)
  - *Serverless Wasm Functions* currently supports only the `default` key value store
- Use the language-specific Spin SDK for interacting with the key value store

The *starting point* contains a function that handles incoming `GET` requests at `/api/ping`. Extend the handler to count its invocations using the key value store.

> **Hint:** Explore the implementation and the application manifest of the *starting point*, you'll find inline comments providing additional context and link to the Spin documentation for further explanation.

### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.

## Task 4: Building multi-component Spin applications

A Spin application could consist of multiple WebAssembly components. As we focus on applications triggered by inbound HTTP requests, each component is linked to a particular path (or path prefix, e.g. `/api/...`). You can use the `spin add` command for adding additional components to an existing Spin application.

Extend the Spin application by adding a new component responsible for serving static content at `/frontend/...`. To do so, complete the following tasks:

- Add a new component called `frontend` to the Spin application using the `static-fileserver` template and set the `HTTP Path` to `/frontend/...` and the asset directory to `frontend`
- Examine the changes applied to the application manifest `spin.toml` you can see another `[[trigger.http]]` being added along the `[comonent.frontend]` block
- Add an `index.html` to the `frontend` sub-folder and provide (or vibe-code) some simple HTML
- Explore [additional configuration variables](https://github.com/spinframework/spin-fileserver?tab=readme-ov-file#configuration-options) for the `static-fileserver` component to tailor the runtime behavior

### Test and Deploy the Spin application

You can test your Spin application at any time (assuming that your code is syntactically correct and compiles) using the `spin up` command. Deploying your Spin application to *Serverless Wasm Functions* is just one `spin aka deploy` away. Redeploying a Spin application will replace the old version. The subdomain generated for your Spin application will not be affected by recurring deployments.
