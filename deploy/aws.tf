terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = "us-east-2"
}


resource "aws_instance" "example" {
  ami           = "ami-097a2df4ac947655f"
  instance_type = "t2.small"
}