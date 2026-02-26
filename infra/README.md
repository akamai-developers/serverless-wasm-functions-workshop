# Cloud Infrastructure

This folder contains a [Terraform](https://terraform.io) project that you could use for deploying the cloud infrastructure required during the workshop. Terraform will provision:

- A Linode instance with a dedicated NVIDIA GPU
  - Necessary drivers and [Ollama](https://ollama.com) will be installed and configured automatically using [the cloud-init script](./userdata/linode.yml)
  - `qwen3` is pulled to the Linode once provisioning is finished
- Firewall is configured to allow inbound connections via HTTP on port `11343` and SSH on port `22`

## Prerequisites

You must have the following tools installed on your machine to use this Terraform project:

- `terraform` CLI (`v1.5.7` or newer)
- `curl`

## Configuring the `linode` provider

You must provide an access token for Linode. Create a new access token with `read/write` permissions for:

- Linode
- Firewall

Aso, `Read` is required for `Events`.

Set the access token as environment variable:

```bash
export LINODE_TOKEN=<YOUR_TOKEN>
```

## Provision the Infrastructure

Run the following commands to initialize and provision the cloud infrastructure:

```bash
terraform init

terraform apply
```
