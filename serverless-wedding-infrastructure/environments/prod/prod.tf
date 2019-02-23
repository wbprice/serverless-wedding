locals {
  environment_code = "prod"
}

provider "aws" {
  region = "us-east-1"
}

module "backend" {
  source = "../../backend"
  environment_code="${local.environment_code}"
}

module "frontend" {
  source = "../../frontend"
}

output "rsvp_table_name" {
    value = "${module.backend.rsvp_table_name}"
}

output "rsvp_table_id_index_name" {
    value = "${module.backend.rsvp_table_id_index_name}"
}

output "api_role_arn" {
    value = "${module.backend.api_role_arn}"
}