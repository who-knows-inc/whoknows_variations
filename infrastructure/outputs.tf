output "azure_dns_zone_nameservers" {
  value = azurerm_dns_zone.keacloud.name_servers
}

output "public_ip_address" {
  value = azurerm_public_ip.keacloud.ip_address
}

output "ssh_command" {
  value = "ssh ${one(azurerm_linux_virtual_machine.keacloud.admin_ssh_key).username}@${azurerm_public_ip.keacloud.ip_address}"
}
