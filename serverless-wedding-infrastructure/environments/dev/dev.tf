provider "aws" {
  region = "us-east-1"
}

module "backend" {
  source = "../../backend"
}