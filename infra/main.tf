locals {
  default_tags = ["hands-on-labs", "workshop", "serverless-wasm-functions", "llm"]
  all_tags     = distinct(concat(local.default_tags, values(var.user_tags)))
}

resource "linode_instance" "llm" {
  label      = "serverless_wasm_workshop_llm"
  image      = "linode/ubuntu24.04"
  region     = var.linode_region
  type       = var.linode_type
  root_pass  = var.root_password
  tags       = local.all_tags
  private_ip = false
  metadata {
    user_data = base64encode(file("./userdata/linode.yml"))
  }
}

data "http" "pull_qwen3" {
  depends_on = [linode_instance.llm]
  url        = "http://${tolist(linode_instance.llm.ipv4)[0]}:11434/api/pull"
  method     = "POST"
  request_headers = {
    "content-type" = "application/json"
  }
  # Optional request body
  request_body = jsonencode({
    model = "qwen3"
  })
}

resource "linode_firewall" "lb-fw" {
  label           = "serverless_wasm_workshop"
  linodes         = [linode_instance.llm.id]
  inbound_policy  = "DROP"
  outbound_policy = "ACCEPT"
  tags            = local.all_tags

  inbound {
    label    = "allow-ssh"
    action   = "ACCEPT"
    protocol = "TCP"
    ports    = "22"
    ipv4     = ["0.0.0.0/0"]
  }

  inbound {
    label    = "allow-ollama-http"
    action   = "ACCEPT"
    protocol = "TCP"
    ports    = "11434"
    ipv4     = ["0.0.0.0/0"]
  }



}
