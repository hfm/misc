terraform {
  backend "local" {}

  required_providers {
    mysql = {
      source = "petoju/mysql"
    }
  }
}

provider "mysql" {
  username = "root"
  password = "root"
}

resource "mysql_database" "test" {
  name = "test${formatdate("YYYYMMDD", timestamp())}"
}

resource "mysql_user" "test" {
  user               = "test"
  host               = "%"
  plaintext_password = "test"
}

resource "mysql_role" "test" {
  name = "test_role"
}

resource "mysql_grant" "test" {
  user     = mysql_user.test.user
  host     = mysql_user.test.host
  database = mysql_database.test.name
  roles    = [mysql_role.test.name]
}
