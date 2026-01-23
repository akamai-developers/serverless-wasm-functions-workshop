# Serverless Wasm Functions Workshop

This repository contains resources for a half-day workshop focussing on *Serverless Wasm Functions*. In addition to a high-level introduction, the workshop consists of three Hands-On-Labs (HOL) exercises. We highly encourage workshop participants to code along and get a sense of how streamlined a developer experience (DX) for building truly serverless edge-native applications feels like in 2026.

## Prerequisites

Workshop participants should bring:

- A GitHub account (The GitHub account is used to authenticate against *Serverless Wasm Functions*)
- An editor or IDE
- Permissions to install software
- Permission to allocate a local port (by default `3000`) for running and testing applications

> If you're not participating an instructor-led version of this workshop, access to Akamai Cloud is required for provisioning a Linode including a dedicated GPU to run large language models (see Hands-On-Labs 3)

## Suggested Running Order

1. Start with the [Introduction](./introduction/) to explain core values and benefits from using WebAssembly to build and run serverless workloads. The presentation provides an high-level introduction to technical underpinnings, introduces [Spin](https://spinframework.dev) and *Serverless Wasm Functions*.

2. Workshop participants [setup their development machines](./developer-machine-setup/) by following the language- and os-specific instructions.

3. Hands-On-Labs 1: [Build & Deploy in 4 Steps](./hol1-build-and-deploy/) is an interactive exercise to familiarize yourself with the developer tooling (`spin` CLI) for building and running WebAssembly applications. As part of this Hands-On-Labs, you will also create your account with *Serverless Wasm Functions* and deploy the first application.

4. Hands-On-Labs 2: [Edge-Native Use Cases](./hol2-edge-native-use-cases/) is another interactive exercise which teaches fundamental building blocks of Spin and *Serverless Wasm Functions*. You will make use of language-specific Spin SDKs to address common day-to-day concerns when building edge-native applications.

5. Hands-On-Labs 3: [Edge Accelerated Generative AI](./hol3-edge-accelerated-genai/) is the final exercise illustrating how you can build smarter applications for the edge by adding generative AI to the mix.

## Hands-On-Labs folder structure and checkpoints

Although Spin supports many different languages, samples are currently provided in [Rust](https://rustlang.org) and [TypeScript](https://www.typescriptlang.org/) only.

With Hands-On-Labs 1, attendees get a chance to start from scratch and learn the "Spin Workflow". Other Hands-On-Labs folders always follow this structure:

```console
hol-folder/
 | - README.md
 | - start-rust/
 | - start-ts/
 | - solution-rust/
 | - solution-ts/
```

Workshop participants could use the corresponding "checkpoint" (`solution-rust` or `solution-ts`) if they have problems with completing a particular Hands-On-Labs. Consider these solution folders as "checkpoints" allowing you (workshop participant) to have the flexibility of extending Hands-On-Labs individually or diving straight into more advanced topics.

## Supportive Cloud Infrastructure

Hands-On-Labs 3 (GenAI at the Edge) requires a Linode (with dedicated GPU) being provisioned and at least one large language model (LLM) being exposed through [Ollama](https://ollama.com/). For instructor-led workshops, workshop participants could expect this infrastructure being pre-provisioned and ready-to-use. (HTTP endpoints will be provided by the workshop instructor)

> The Cloud infrastructure setup instructions could be found in the [`infra`](./infra/) folder.

## Cleaning up your Serverless Wasm Functions Account

As part of this workshop, you will deploy several applications to your individual *Serverless Wasm Functions* account. To retrieve a list of applications deployed to your account, can always use the `spin aka apps list` command.

To delete applications deployed to your account, run the following commands:

```bash
spin aka apps delete --app-name hello-serverless-functions --no-confirm
spin aka apps delete --app-name edge-native-use-cases --no-confirm
spin aka apps delete --app-name edge-accelerated-genai --no-confirm
```