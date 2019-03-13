variable "environment_code" {
    type = "string"
    default = "dev"
}

output "rsvp_table_name" {
    value = "${aws_dynamodb_table.rsvp_table.name}"
}

output "rsvp_table_id_index_name" {
    value = "${local.rsvp_table_id_index_name}"
}

output "rsvp_table_stream_arn" {
    value = "${aws_dynamodb_table.rsvp_table.stream_arn}"
}

output "api_role_arn" {
    value = "${aws_iam_role.serverless_wedding_api_iam_role.arn}"
}

output "rsvp_handler_role_arn" {
    value = "${aws_iam_role.serverless_wedding_rsvp_handler_iam_role.arn}"
}