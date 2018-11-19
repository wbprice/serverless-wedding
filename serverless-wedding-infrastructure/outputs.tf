output "rsvp_table" {
    value = "${aws_dynamodb_table.rsvp-table.arn}"
}