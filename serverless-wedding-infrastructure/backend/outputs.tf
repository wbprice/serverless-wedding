output "rsvp_table_name" {
    value = "${aws_dynamodb_table.rsvp_table.name}"
}

output "api_role_arn" {
    value = "${aws_iam_role.serverless_wedding_api_iam_role.arn}"
}