# Serverless Wasm Functions Workshop

This repository contains resources for a half-day workshop focussing on *Serverless Wasm Functions*. In addition to a high-level introduction, the workshop consists of three Hands-On-Labs (HOL) exercises. We highly encourage workshop attendees to code along to get a sense of how streamlined a developer experience for building truly serverless edge-native applications feel in 2026. 

## Prerequisites

Attendees should have

- A GitHub account (The GitHub account is used to authenticate against *Serverless Wasm Functions*)
- A text editor installed on their machine
- Permissions to install software on their machine (As part of the first HOL, users will install language-specific tooling and the `spin` CLI) 
- Permission to allocate a local port (by default `3000`) for running and testing apps created during HOL on their local machine
- If you're not participating an instructor-led version of this workshop, access to Akamai Cloud is required for provisioning a Linde including a dedicated GPU to run large language models (see HOL 3)

## Suggested Running Order

1. Start with the [Introduction](./introduction/) to explain core values and benefits from using WebAssembly to build and run serverless workloads. The slide deck provides an high-level introduction to technical underpinnings and introduces [Spin](https://spinframework.dev) and *Serverless Wasm Functions* to the audience.

2. HOL 1: [Build & Deploy in 4 Steps](./hol1-build-and-deploy/) is an interactive exercise to help developers familiarizing themselves with developer tooling (`spin` CLI) for building and running their first application. As part of this HOL, attendees will also create their account with *Serverless Wasm Functions* and deploy their workload.

3. HOL 2: [Edge-Native Use Cases](./hol2-edge-native-use-cases/) is another interactive exercise which teaches fundamental building blocks of Spin and *Serverless Wasm Functions*. Developers will make use of language-specific Spin SDKs to address common day-to-day concerns when building edge-native applications.

4. HOL 3: [Generative AI at the Edge](./hol3-genai-at-the-edge/) is the final exercise illustrating how developers can build smart applications for the edge by adding generative AI to the mix


## HOL folder structure and checkpoints

Although Spin supports many different languages, samples are currently provided in [Rust](https://rustlang.org) and [TypeScript](https://www.typescriptlang.org/) only.

With HOL 1, attendees get a chance to start from scratch and learn the "Spin Workflow". Other HOL folders always follow this structure:

```console
hol-folder/
 | - README.md
 | - start-rust/
 | - start-ts/
 | - solution-rust/
 | - solution-ts/
```

Each `solution-*` folder helps attendees to unblock themselves if they are facing errors. Consider these solution folders as "check points" giving the attendees also some flexibility when it comes to extending HOLs individually to dive into more advanced topics.

## Supportive Cloud Infrastructure

HOL 3 (GenAI at the Edge) requires a Linode (with dedicated GPU) being provisioned and at lest one LLM being exposed through [Ollama](https://ollama.com/). For instructor-led workshops, attendees should expect this infrastructure being pre-provisioned and ready-to-use. 

> Cloud Infrastructure setup instructions are located in the [`infra` folder](./infra/)


## Cleaning up your Serverless Wasm Functions Account

As part of this workshop, you'll deploy XXX applications to your individual *Serverless Wasm Functions* account. To retrieve a list of applications deployed to your account, can always use the `spin aka apps list` command.

To delete applications deployed to your account as part of this workshop, run the following commands:

```bash
spin aka apps delete --app-name hello-serverless-functions --no-confirm
spin aka apps delete --app-name edge-native-use-cases --no-confirm
spin aka apps delete --app-name genai-at-the-edge --no-confirm
```