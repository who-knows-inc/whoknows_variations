resource "azurerm_virtual_network" "keacloud" {
  name                = var.vnet_name
  address_space       = [var.vnet_address_space]
  location            = azurerm_resource_group.keacloud.location
  resource_group_name = azurerm_resource_group.keacloud.name
}

resource "azurerm_subnet" "keacloud" {
  name                 = var.subnet_name
  resource_group_name  = azurerm_resource_group.keacloud.name
  virtual_network_name = azurerm_virtual_network.keacloud.name
  address_prefixes     = [var.subnet_address_prefix]
}

resource "azurerm_public_ip" "keacloud" {
  name                = var.public_ip_name
  location            = azurerm_resource_group.keacloud.location
  resource_group_name = azurerm_resource_group.keacloud.name
  allocation_method   = var.public_ip_allocation_method
}

resource "azurerm_network_interface" "keacloud" {
  name                = var.nic_name
  location            = azurerm_resource_group.keacloud.location
  resource_group_name = azurerm_resource_group.keacloud.name

  ip_configuration {
    name                          = var.ip_configuration_name
    subnet_id                     = azurerm_subnet.keacloud.id
    private_ip_address_allocation = var.private_ip_allocation
    public_ip_address_id          = azurerm_public_ip.keacloud.id
  }
}

resource "azurerm_network_security_group" "keacloud_nsg" {
  name                = "keacloud-nsg"
  location            = azurerm_resource_group.keacloud.location
  resource_group_name = azurerm_resource_group.keacloud.name


  security_rule {
    name                       = "allow-80"
    priority                   = 100
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "80"
    source_address_prefix      = "*"
    destination_address_prefix = "*"
  }

}

resource "azurerm_network_security_rule" "keacloud_ssh_rule" {
  name                        = "SSH"
  priority                    = 1000
  direction                   = "Inbound"
  access                      = "Allow"
  protocol                    = "Tcp"
  source_port_range           = "*"
  destination_port_range      = "22"
  source_address_prefix       = "*"
  destination_address_prefix  = "*"
  network_security_group_name = azurerm_network_security_group.keacloud_nsg.name
  resource_group_name         = azurerm_resource_group.keacloud.name
}

resource "azurerm_dns_a_record" "keacloud" {
  name                = "www"
  zone_name           = azurerm_dns_zone.keacloud.name
  resource_group_name = azurerm_resource_group.keacloud.name
  ttl                 = 300
  records             = [azurerm_public_ip.keacloud.ip_address]
}
