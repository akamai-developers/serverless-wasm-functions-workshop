terraform {
  required_providers {
    http = {
      source  = "hashicorp/http"
      version = "3.5.0"
    }
    linode = {
      source  = "linode/linode"
      version = "3.7.0"
    }
  }
}

provider "linode" {}
provider "http" {}