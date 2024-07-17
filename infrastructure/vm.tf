resource "azurerm_linux_virtual_machine" "keacloud" {
  name                = var.vm_name
  resource_group_name = azurerm_resource_group.keacloud.name
  location            = azurerm_resource_group.keacloud.location
  size                = var.vm_size
  admin_username      = var.admin_username
  network_interface_ids = [
    azurerm_network_interface.keacloud.id,
  ]

  os_disk {
    caching              = var.os_disk_caching
    storage_account_type = var.os_disk_storage_account_type
  }

  source_image_reference {
    publisher = var.source_image_publisher
    offer     = var.source_image_offer
    sku       = var.source_image_sku
    version   = var.source_image_version
  }

  admin_ssh_key {
    username   = var.admin_username
    public_key = file(var.ssh_public_key)
  }

  disable_password_authentication = true

  provisioner "file" {
    source      = var.ansible_playbook_source
    destination = var.ansible_playbook_destination

    connection {
      type        = "ssh"
      user        = var.admin_username
      private_key = file(var.ssh_private_key)
      host        = azurerm_public_ip.keacloud.ip_address
    }
  }

  provisioner "file" {
    source      = var.ssh_private_key
    destination = "/home/${var.admin_username}/.ssh/id_rsa"

    connection {
      type        = "ssh"
      user        = var.admin_username
      private_key = file(var.ssh_private_key)
      host        = azurerm_public_ip.keacloud.ip_address
    }
  }

  provisioner "remote-exec" {
    inline = split("\n", templatefile("${path.module}/inline_commands.sh", {}))

    connection {
      type        = "ssh"
      user        = var.admin_username
      private_key = file(var.ssh_private_key)
      host        = self.public_ip_address
      timeout     = "2m"
    }
  }
}