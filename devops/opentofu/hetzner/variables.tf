variable "project_name" {
  type        = string
  description = "Name prefix for resources"
  default     = "climatenews"
}

variable "hcloud_token" {
  type        = string
  description = "Hetzner Cloud API token"
  sensitive   = true
}

variable "hetzner_dns_api_token" {
  type        = string
  description = "Hetzner DNS API token"
  sensitive   = true
  default     = ""
}

variable "server_type" {
  type        = string
  description = "Hetzner server type (for example: cpx21)"
  default     = "cpx21"
}

variable "server_image" {
  type        = string
  description = "OS image name"
  default     = "ubuntu-24.04"
}

variable "location" {
  type        = string
  description = "Hetzner location (for example: hel1, nbg1, fsn1, ash)"
  default     = "hel1"
}

variable "ssh_public_key_path" {
  type        = string
  description = "Absolute path to the public SSH key used for server login"
}

variable "ssh_allowed_cidr" {
  type        = string
  description = "CIDR allowed to reach SSH port 22"
}

variable "base_domain_name" {
  type        = string
  description = "Apex domain name"
  default     = "climatenews.app"
}

variable "manage_dns" {
  type        = bool
  description = "Create and manage DNS records in Hetzner DNS"
  default     = true
}

variable "mx_records" {
  type        = list(string)
  description = "MX record values"
  default = [
    "10 mx1.privateemail.com",
    "10 mx2.privateemail.com"
  ]
}

variable "txt_records" {
  type        = list(string)
  description = "TXT record values"
  default = [
    "v=spf1 include:spf.privateemail.com ~all"
  ]
}
