# Developer Machine Setup 

First, we want to ensure your development machine is good to go. Although Spin (the developer tool for building WebAssembly applications) supports multiple languages, we'll only provide instructions for Rust and TypeScript as part of this workshop.

If you want to explore Spin using a different programming language, consult the official Spin documentation at [https://spinframework.dev](https://spinframework.dev). It contains language guides for [Go](https://spinframework.dev/v3/go-components) and [Python](https://spinframework.dev/v3/python-components) as well.


### Rust Language Guide

Install [Rust](https://rustlang.org) on your machine. Although there are plenty ways for installing Rust, we recommend using Rustup. Rustup is the Rust installer and version management tool:

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