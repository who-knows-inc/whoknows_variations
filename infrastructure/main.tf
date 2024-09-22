terraform {
  required_version = ">= 1.2.0"

  required_providers {
    azurerm = {
      source  = "hashicorp/azurerm"
      version = "3.83.0"
    }
  }
}

provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "keacloud" {
  name     = var.resource_group_name
  location = var.location
}