terraform {
  required_version = "= 1.3.6"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.47.0"
    }
  }
}

provider "aws" {
  region = local.region
  default_tags {
    tags = {
      ManagedBy = "Terraform"
      Scope = local.app_name
    }
  }
}
