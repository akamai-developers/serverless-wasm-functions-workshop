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

  provisioner "local-exec" {
    command = "curl --retry 15 --retry-delay 30 --retry-connrefused -H 'Content-Type: application/json' -d '{\"model\": \"llama3.2:1b\"}' http://${tolist(self.ipv4)[0]}:11434/api/pull"
  }
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
