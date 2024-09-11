terraform {
  backend "local" {}

  required_providers {
    mysql = {
      source = "petoju/mysql"
    }
  }
}

provider "mysql" {
  endpoint = "127.0.0.1:3306"
  username = "root"
}

data "mysql_databases" "dbs" {
  pattern = "%"
}

output "dbs" {
  value = data.mysql_databases.dbs.databases
}
