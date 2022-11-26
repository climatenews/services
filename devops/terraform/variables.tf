variable "aws_region" {
  default = "us-east-2"
}

variable "aws_availability_zone" {
  default = "us-east-2a"
}

variable "aws_key_name" {
  default = "ubuntu_desktop"
}

variable "aws_pem_file" {
  default = "~/.ssh/ubuntu_desktop.pem"
}

variable "aws_instance_type" {
  default = "t2.small"
}

variable "aws_base_ami" {
  default = "ami-097a2df4ac947655f"
}

variable "base_domain_name" {
  default = "climatenews.io"
}

variable "terraform_state_bucket" {
  default = "climate-news-terraform-state"
}

variable "terraform_state_lock_table" {
  default = "climate-news-terraform-state-lock-table"
}
