# Developer Machine Setup

First, we want to ensure your development machine is good to go. Although Spin (the developer tool for building WebAssembly applications) supports multiple languages, we'll only provide instructions for Rust and TypeScript as part of this workshop.

If you want to explore Spin using a different programming language, consult the official Spin documentation at [https://spinframework.dev](https://spinframework.dev). It contains language guides for [Go](https://spinframework.dev/v3/go-components) and [Python](https://spinframework.dev/v3/python-components) as well.

## Rust Language Guide

Install [Rust](https://rustlang.org) on your machine. Although there are plenty ways for installing Rust, we recommend using `rustup`. `rustup` is the Rust installer and version management tool:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you've stable Rust installed on your machine, you must install the `wasm32-wasip1` target for Rust:

```bash
rustup target add wasm32-wasip1
```

---

## TypeScript Language Guide

For building Spin application in TypeScript (or plain JavaScript) you must have [Node.js](https://nodejs.org/en) installed on your system. We recommend using latest LTS (Krypton `24.12.0` at the time of writing this document).

There are plenty ways of installing and managing Node.js version. We highly recommend using a version manager such as [`nvm`](https://github.com/nvm-sh/nvm) for example. By using a Node.js version manager, you are able to switch Node.js versions on the fly:

```bash
# download and install Node.js 24 (latest LTS)
nvm install 24

# set Node.js 24 as current node version
nvm use 24
```

If you don't want to use `nvm`, you can find and install Node.js at [nodejs.org](https://nodejs.org/en).

---

## Installing Spin CLI

Spin provides a language agnostic CLI addressing all inner-loop concerns in an elegant and highly-productive fashion. No matter if you're using Rust, TypeScript or any other supported language, you need only a handful of simple commands for creating, compiling, running, and deploying your applications.

That's what we call the **Spin Workflow**:

- `spin new`: For creating new applications from templates
- `spin build`: For compiling your source down to WebAssembly
- `spin up`: For running your application straight on your machine
- `spin aka deploy`: For deploying your application to Serverless Wasm Functions

Spin has more to offer, you can create and distribute your own templates, write plugins for extending its capabilities or seamlessly integrating it into your existing suite of tools.

Let's put hype and enthusiasm aside and install `spin` on your machine now.

### Installing Spin CLI on macOS

To install Spin CLI on macOS, you can either use Homebrew or the Spin installer script:

#### Homebrew

You can manage your Spin installation via Homebrew. Homebrew automatically installs Spin templates and Spin plugins, and on uninstall, will prompt you to delete the directory where the templates and plugins were downloaded:

Install the Spin Framework tap, which Homebrew tracks, updates, and installs Spin from:

```bash
brew tap spinframework/tap
```

Install Spin:

```bash
brew install spinframework/tap/spin
```

#### Installer script (macOS)

Another option (other than `brew`) is to use the Spin installer script. The installer script installs Spin along with a starter set of language templates and plugins:

```bash
curl -fsSL https://spinframework.dev/downloads/install.sh | bash
```

Once you have run the installer script, it is highly recommended to add Spin to a folder, which is on your path, e.g.:

```bash
sudo mv spin /usr/local/bin/
```

### Installing Spin CLI on Linux

The installer script installs Spin along with a starter set of language templates and plugins:

```bash
curl -fsSL https://spinframework.dev/downloads/install.sh | bash
```

Once you have run the installer script, it is highly recommended to add Spin to a folder, which is on your path, e.g.:

```bash
sudo mv spin /usr/local/bin/
```

### Installing Spin CLI on Windows

If using Windows (PowerShell / `cmd.exe`), you can download the [Windows binary release of Spin](https://github.com/spinframework/spin/releases/latest).

Simply unzip the binary release and place the `spin.exe` in your system path.

This does not install any Spin templates or plugins. For a starter list, see the [Installing Templates and Plugins section of the Spin documentation](https://spinframework.dev/v3/install#installing-templates-and-plugins).

If you want to use WSL2 (Windows Subsystem for Linux 2), please follow the [instructions for using Linux](#installing-spin-cli-on-linux).

## Installing the `aka` plugin for Spin CLI

As part of this workshop, you'll deploy several Spin applications to *Serverless Wasm Functions*. As a developer, you'll mainly interact with *Serverless Wasm Functions* through the CLI interface provided by the `aka` plugin for Spin CLI.

You can find and install the `aka` plugin (along many other useful plugins for Spin CLI) using the sub-commands of `spin plugins`. 

To install the `aka` plugin, execute the following command:

```bash
spin plugins install aka --yes
```

You can verify the installation of the `aka` plugin by simply executing `spin aka --version`, (which should print `0.6.0` or higher).

---

🎉 That's all you need to start you WebAssembly journey 🎉

[Go back to the workshop instructions](/README.md#suggested-running-order)
