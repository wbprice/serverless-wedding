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
    value = "${module.backend.rsvp_table_name}"
}

output "rsvp_table_id_index_name" {
    value = "${module.backend.rsvp_table_id_index_name}"
}

output "rsvp_table_stream_arn" {
  value = "${module.backend.rsvp_table_stream_arn}"
}

output "api_role_arn" {
    value = "${module.backend.api_role_arn}"
}

output "rsvp_handler_role_arn" {
    value = "${module.backend.rsvp_handler_role_arn}"
}

output "rsvp_handler_sns_topic_arn" {
    value = "${module.backend.rsvp_handler_sns_topic_arn}"
}