# OpenTofu Hetzner Stack

This folder provisions a single Hetzner VM with a reserved IPv4, firewall rules, and optional Hetzner DNS records.

## What it creates

- One `hcloud_server` for app workloads
- One reserved `hcloud_primary_ip` attached to the server
- One `hcloud_firewall` allowing SSH (restricted), HTTP, and HTTPS
- Optional Hetzner DNS zone/records (apex A, www CNAME, MX, TXT)

## Prerequisites

- OpenTofu installed
- Hetzner Cloud API token
- Hetzner DNS API token (only if `manage_dns = true`)
- Existing SSH public key file

## Quick start

1. Copy the example variables file:

   cp terraform.tfvars.example terraform.tfvars

2. Set secrets as environment variables:

   export TF_VAR_hcloud_token="<token>"
   export TF_VAR_hetzner_dns_api_token="<token>"

3. Initialize and plan:

   tofu init
   tofu plan

4. Apply:

   tofu apply

5. Get server IP:

   tofu output primary_ipv4

## Next steps

- Update `devops/ansible/production.yml` with the new host/IP
- Deploy Docker services to the server
- Add DB backup automation (pgBackRest or WAL-G)
