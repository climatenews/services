output "server_id" {
  description = "Hetzner server ID"
  value       = hcloud_server.app.id
}

output "server_name" {
  description = "Hetzner server name"
  value       = hcloud_server.app.name
}

output "primary_ipv4" {
  description = "Primary IPv4 address for app host"
  value       = hcloud_primary_ip.ipv4.ip_address
}

output "server_ipv6" {
  description = "Server IPv6 network"
  value       = hcloud_server.app.ipv6_address
}

output "zone_id" {
  description = "Hetzner DNS zone ID when manage_dns is enabled"
  value       = var.manage_dns ? hetznerdns_zone.primary[0].id : null
}
