# HOL Instructions | Build & Deploy in Three Steps

First, we want to ensure your development machine is good to go. Although Spin (the developer tool for building WebAssembly applications) supports multiple languages, we'll only provide instructions for Rust and TypeScript as part of this workshop.

## Development Machine Setup 

If you want to explore Spin using a different programming language, consult the official Spin documentation at [https://spinframework.dev](https://spinframework.dev). It contains language guides for [Go](https://spinframework.dev/v3/go-components) and [Python](https://spinframework.dev/v3/python-components) as well.


### Rust Language Guide

Install [Rust](https://rustlang.org) on your machine. Although there are plenty ways for installing Rust, we recommend using Rustup. Rustup is the Rust installer and version management tool.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you've stable Rust installed on your machine, you must install the `wasm32-wasip1` target for Rust:

```bash
rustup target add wasm32-wasip1
```

### TypeScript Language Guide

For building Spin application in TypeScript (or plain JavaScript) you must have Node.JS installed on your system. We recommend using latest LTS (Krypton `24.12.0` at the time of writing this document). 

There are plenty ways of installing and managing Node.JS version. We highly recommend using a version manager such as [`nvm`](https://github.com/nvm-sh/nvm) for example. By using a node version manager, you are able to switch Node.JS versions on the fly:

```bash
# download and install Node.JS 24 (latest LTS)
nvm install 24

# set Node.JS 24 as current node version
nvm use 24
```

If you don't want to use `nvm`, you can find and install Node.JS at [nodejs.org](https://nodejs.org/en).

### Install Spin CLI

Spin provides a language agnostic CLI addressing all inner-loop concerns in an elegant and highly-productive fashion. No matter if you're using Rust, TypeScript or any other supported language, you need only a handful of simple commands for creating, compiling, running, and deploying your applications. 

That's what we call the **Spin Workflow**:

- `spin new`: For creating new applications from templates
- `spin build`: For compiling your source down to WebAssembly
- `spin up`: For running your application straight on your machine
- `spin aka deploy`: For deploying your application to Serverless Wasm Functions

Spin has more to offer, you can create and distribute your own templates, write plugins for extending its capabilities or seamlessly integrating it into your existing suite of tools.

Let's put hype and enthusiasm aside and install `spin` on your machine now.

#### Installing Spin CLI on macOS

TBD

#### Installing Spin CLI on Linux

TBD

#### Installing Spin CLI on Windows

TBD

🎉That's all you need to start you WebAssembly journey 🎉

## Build & Deploy in Four Steps

Having installed all necessary things on your development machine, it's time to get started with Spin, learn the Spin workflow and build your first application. 

### Step 1: Create your first Spin application using `spin new`

Navigate to the [`hol1-build-and-deploy` folder](./) and create your first Spin application using `spin new`. The `spin new` command will present a list of installed templates. As we'll build a simple HTTP based function, select either `http-rust` or `http-ts` depending on your preferred programming language. 

The wizard asks for fundamental metadata like `name`, `description`, `HTTP Path` and (in case of TypeScript) for your preferred HTTP router. Use the values shown in the following table:

| Metadata Field | Condition | Value to Provide | Description |
|----------------|-----------|------------------|-------------|
| Name           |           | `hello-serverless-functions` | |
| Description    |           | `` | just confirm with `RETURN` |
| HTTP Path      |           | `/...` | just confirm with `RETURN` |
| HTTP Router    | TS/JS only | `hono` | You can also choose `itty` or `none` but our solution is built with `hono` |

> Spin commands are tweaked for productivity and are provide all sorts of flags to fully automate them when executed in an unattended environment. That said, you could also use the `spin new -t http-rust -a hello-serverless-functions` (or respectively `spin new -t http-ts --value http-router=hono -a hello-serverless-functions`) command for creating the application with the same configuration

### Step 2 Compile the source code down to WebAssembly with `spin build`

Move into the application directory `cd hello-serverless-functions` and execute `spin build`.

```bash
spin build

Building component hello-serverless-functions
# ...
Finished building all Spin components
```

### Step 3: Run the application on your local machine using `spin up`

```bash
spin up
Logging component stdio to ".spin/logs/"
Preparing Wasm modules is taking a few seconds...


Serving http://127.0.0.1:3000
Available Routes:
  hello-serverless-functions: http://127.0.0.1:3000 (wildcard)
```

From within a new terminal instance, send an HTTP `GET` request to the root route of your application running at `localhost:3000`:

```bash
curl -i localhost:3000/

HTTP/1.1 200 OK
content-type: text/plain; charset=UTF-8
server: Spin CLI
content-length: 12
date: Fri, 09 Jan 2026 15:12:24 GMT

Hello, Spin!
```

### Step 4: Deploy the application to Serverless Wasm Functions using `spin aka deploy`

#### Authenticating

As of now, Serverless Wasm Functions provides a gated onboarding process. Your workshop instructor should already have asked for your GitHub username. If not, please let your instructor immediately know. Alternatively, you can fill the following form to get an account provisioned: [Get an Serverless Wasm Functions Account](https://fibsu0jcu2g.typeform.com/fwf-preview).

Once your account has been provisioned (you'll receive a confirmation by mail), you can authenticate using the `spin aka login` command. The command will trigger a browser based authentication flow asking you for permission to interact with Serverless Wasm Functions on your behalf. Grant Spin permission for doing so.

```bash
spin aka login
Go to https://login.infra.fermyon.tech/realms/neutrino/device?user_code=0000-0000 and follow the prompts.

Don't worry, we'll wait here for you. You got this.

Welcome, <YOUR_GITHUB_USERNAME>.
```

---

Once authenticated, you can deploy, browse, inspect, observe and remove Spin applications from your individual Serverless Wasm Functions account. Let's finally deploy the `hello-serverless-functions` application:

```bash
spin aka deploy
Name of new app: hello-serverless-functions # confirm with RETURN
Note: If you would instead like to deploy to an existing app, cancel this deploy and link this workspace to the app with `spin aka app link`
OK to continue? [Y/n] # confirm with RETURN
```

When deploying for the first time, `spin aka deploy` command asks about the desired name and for confirmation to actually deploy. Again, there are flags to deploy without the CLI asking those questions: `spin aka deploy --create-name hello-serverless-functions --no-confirm `.

**Deploying an application to Serverless Wasm Functions** takes roughly 50 seconds in which your application is uploaded to Serverless Wasm Functions, its deployed throughout all service regions across the globe and a random endpoint is generated for your workload. 🚀

```bash
spin aka deploy --no-confirm --create-name hello-serverless-functions
Workspace linked to app hello-serverless-functions
Waiting for app to be ready... ready

App Routes:
- hello-serverless-functions: https://10663d58-fea4-437f-a323-6e493caff337.fwf.app (wildcard)
```

Once the command has finished, you can sent another HTTP `GET` request. However, this time we'll sent it to the endpoint generated for your application on top of Serverless Wasm Functions:

```bash
curl -i https://10663d58-fea4-437f-a323-6e493caff337.fwf.app

HTTP/1.1 200 OK
Content-Type: text/plain; charset=UTF-8
Server: envoy
Content-Length: 12
x-envoy-upstream-service-time: 58
Date: Fri, 09 Jan 2026 15:26:02 GMT
Connection: keep-alive
Akamai-Request-BC: [a=188.210.40.197,b=501999255,c=g,n=DE_SL_SAARLOUIS,o=42652],[a=197,c=o]
Set-Cookie: akaalb_fwf-prod-apps=~op=fwf_prod:fwf-dev-de-fra-2|~rv=48~m=fwf-dev-de-fra-2:0|~os=1231e1ede8704e97468b2ddc2c84cd5b~id=d750418fa3e88716c36de27ffe2b1349; path=/; HttpOnly; Secure; SameSite=None
Akamai-GRN: 0.c528d2bc.1767972362.1debe697

Hello, Spin!
```