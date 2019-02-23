locals {
  environment_code = "dev"
}

provider "aws" {
  region = "us-east-1"
}

module "backend" {
  source = "../../backend"
  environment_code="${local.environment_code}"
}

output "rsvp_table_name" {
    value = "${backend.rsvp_table.name}"
}

output "rsvp_table_id_index_name" {
    value = "${backend.rsvp_table_id_index_name}"
}

output "api_role_arn" {
    value = "${backend.serverless_wedding_api_iam_role.arn}"
}