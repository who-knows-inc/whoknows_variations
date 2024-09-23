resource "azurerm_virtual_network" "whoknows" {
  name                = var.vnet_name
  address_space       = [var.vnet_address_space]
  location            = azurerm_resource_group.whoknows.location
  resource_group_name = azurerm_resource_group.whoknows.name
}

resource "azurerm_subnet" "whoknows" {
  name                 = var.subnet_name
  resource_group_name  = azurerm_resource_group.whoknows.name
  virtual_network_name = azurerm_virtual_network.whoknows.name
  address_prefixes     = [var.subnet_address_prefix]
}

resource "azurerm_public_ip" "whoknows" {
  name                = var.public_ip_name
  location            = azurerm_resource_group.whoknows.location
  resource_group_name = azurerm_resource_group.whoknows.name
  allocation_method   = var.public_ip_allocation_method
}

resource "azurerm_network_interface" "whoknows" {
  name                = var.nic_name
  location            = azurerm_resource_group.whoknows.location
  resource_group_name = azurerm_resource_group.whoknows.name

  ip_configuration {
    name                          = var.ip_configuration_name
    subnet_id                     = azurerm_subnet.whoknows.id
    private_ip_address_allocation = var.private_ip_allocation
    public_ip_address_id          = azurerm_public_ip.whoknows.id
  }
}

resource "azurerm_network_security_group" "whoknows_nsg" {
  name                = "whoknows-nsg"
  location            = azurerm_resource_group.whoknows.location
  resource_group_name = azurerm_resource_group.whoknows.name


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

  security_rule {
    name                       = "allow-443"
    priority                   = 101
    direction                  = "Inbound"
    access                     = "Allow"
    protocol                   = "Tcp"
    source_port_range          = "*"
    destination_port_range     = "443"
    source_address_prefix      = "*"
    destination_address_prefix = "*"
  }

}

resource "azurerm_network_security_rule" "whoknows_ssh_rule" {
  name                        = "SSH"
  priority                    = 1000
  direction                   = "Inbound"
  access                      = "Allow"
  protocol                    = "Tcp"
  source_port_range           = "*"
  destination_port_range      = "22"
  source_address_prefix       = "*"
  destination_address_prefix  = "*"
  network_security_group_name = azurerm_network_security_group.whoknows_nsg.name
  resource_group_name         = azurerm_resource_group.whoknows.name
}
