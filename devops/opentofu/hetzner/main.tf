locals {
  name = var.project_name
}

resource "hcloud_ssh_key" "default" {
  name       = "${local.name}-ssh-key"
  public_key = file(var.ssh_public_key_path)
}

resource "hcloud_primary_ip" "ipv4" {
  name          = "${local.name}-ipv4"
  datacenter    = upper(var.location)
  type          = "ipv4"
  assignee_type = "server"
  auto_delete   = false
}

resource "hcloud_firewall" "main" {
  name = "${local.name}-firewall"

  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "22"
    source_ips = [var.ssh_allowed_cidr]
  }

  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "80"
    source_ips = ["0.0.0.0/0", "::/0"]
  }

  rule {
    direction  = "in"
    protocol   = "tcp"
    port       = "443"
    source_ips = ["0.0.0.0/0", "::/0"]
  }
}

resource "hcloud_server" "app" {
  name        = "${local.name}-app"
  server_type = var.server_type
  image       = var.server_image
  location    = var.location
  ssh_keys    = [hcloud_ssh_key.default.id]

  public_net {
    ipv4_enabled = true
    ipv6_enabled = true
    ipv4         = hcloud_primary_ip.ipv4.id
  }

  labels = {
    app = local.name
    env = "prod"
  }
}

resource "hcloud_firewall_attachment" "main" {
  firewall_id = hcloud_firewall.main.id
  server_ids  = [hcloud_server.app.id]
}

resource "hetznerdns_zone" "primary" {
  count = var.manage_dns ? 1 : 0
  name  = var.base_domain_name
  ttl   = 60
}

resource "hetznerdns_record" "apex_a" {
  count   = var.manage_dns ? 1 : 0
  zone_id = hetznerdns_zone.primary[0].id
  name    = "@"
  type    = "A"
  value   = hcloud_primary_ip.ipv4.ip_address
  ttl     = 60
}

resource "hetznerdns_record" "www_cname" {
  count   = var.manage_dns ? 1 : 0
  zone_id = hetznerdns_zone.primary[0].id
  name    = "www"
  type    = "CNAME"
  value   = var.base_domain_name
  ttl     = 300
}

resource "hetznerdns_record" "mx" {
  count   = var.manage_dns ? length(var.mx_records) : 0
  zone_id = hetznerdns_zone.primary[0].id
  name    = "@"
  type    = "MX"
  value   = var.mx_records[count.index]
  ttl     = 3600
}

resource "hetznerdns_record" "txt" {
  count   = var.manage_dns ? length(var.txt_records) : 0
  zone_id = hetznerdns_zone.primary[0].id
  name    = "@"
  type    = "TXT"
  value   = var.txt_records[count.index]
  ttl     = 300
}
