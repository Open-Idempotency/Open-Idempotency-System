terraform {

  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "~> 4.15"
    }
  }

  backend "s3" {
    bucket = "tfstate-3ea6z45i"
    key    = "IDP/key"
    region = "us-east-2"
    dynamodb_table = "app-state"
    encrypt = true
  }
}

provider "aws" {
  region = "us-east-2"

}
