variable "resource_group_name" {
  description = "The name of the resource group"
  type        = string
}

variable "location" {
  description = "The Azure location where the resources will be deployed"
  type        = string
}

variable "vnet_name" {
  description = "The name of the virtual network"
  type        = string
  default     = "keacloud-vnet"
}

variable "vnet_address_space" {
  description = "The address space for the virtual network"
  type        = string
  default     = "10.0.0.0/16"
}

variable "subnet_name" {
  description = "The name of the subnet"
  type        = string
  default     = "internal"
}

variable "subnet_address_prefix" {
  description = "The address prefix for the subnet"
  type        = string
  default     = "10.0.2.0/24"
}

variable "public_ip_name" {
  description = "The name of the public IP"
  type        = string
  default     = "keacloud-publicip"
}

variable "public_ip_allocation_method" {
  description = "The allocation method for the public IP"
  type        = string
  default     = "Static"
}

variable "nic_name" {
  description = "The name of the network interface"
  type        = string
  default     = "keacloud-nic"
}

variable "ip_configuration_name" {
  description = "The name of the IP configuration"
  type        = string
  default     = "internal"
}

variable "private_ip_allocation" {
  description = "The allocation method for the private IP"
  type        = string
  default     = "Dynamic"
}

variable "vm_name" {
  description = "The name of the virtual machine"
  type        = string
  default     = "main-vm"
}

variable "vm_size" {
  description = "The size of the virtual machine"
  type        = string
  default     = "Standard_B1s"
}

variable "admin_username" {
  description = "The admin username for the virtual machine"
  type        = string
  default     = "adminuser"
}

variable "os_disk_caching" {
  description = "The caching setting for the OS disk"
  type        = string
  default     = "ReadWrite"
}

variable "os_disk_storage_account_type" {
  description = "The storage account type for the OS disk"
  type        = string
  default     = "Standard_LRS"
}

variable "source_image_publisher" {
  description = "The publisher of the source image"
  type        = string
  default     = "Canonical"
}

variable "source_image_offer" {
  description = "The offer of the source image"
  type        = string
  default     = "UbuntuServer"
}

variable "source_image_sku" {
  description = "The SKU of the source image"
  type        = string
  default     = "18.04-LTS"
}

variable "source_image_version" {
  description = "The version of the source image"
  type        = string
  default     = "latest"
}

variable "ssh_public_key" {
  description = "The path to the SSH public key"
  type        = string
  default     = "~/.ssh/id_rsa.pub"
}

variable "ssh_private_key" {
  description = "The path to the SSH private key"
  type        = string
  default     = "~/.ssh/id_rsa"
}

variable "ansible_playbook_source" {
  description = "The source path for the Ansible playbook"
  type        = string
  default     = "./ansible/playbook.yml"
}

variable "ansible_playbook_destination" {
  description = "The destination path for the Ansible playbook"
  type        = string
  default     = "/home/adminuser/playbook.yml"
}