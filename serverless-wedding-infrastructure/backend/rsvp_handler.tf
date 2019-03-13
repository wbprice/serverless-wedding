data "template_file" "serverless_wedding_rsvp_handler_assume_role_policy" {
    template = "${file("${path.module}/iam/policies/assume-role-policy.json")}"
}

resource "aws_iam_role" "serverless_wedding_rsvp_handler_iam_role" {
    name = "serverless-wedding-rsvp-handler-${var.environment_code}-iam-role"

    assume_role_policy = "${data.template_file.serverless_wedding_rsvp_handler_assume_role_policy.rendered}"
}

data "template_file" "serverless_wedding_rsvp_handler_iam_policy" {
    template = "${file("${path.module}/iam/policies/rsvp-handler-policy.json")}"

    vars = {
        rsvp_table_stream_arn = "${aws_dynamodb_table.rsvp_table.stream_arn}"
        rsvp_handler_sns_topic = "${aws_sns_topic.rsvp_update_notifications.arn}"
    }
}

resource "aws_iam_role_policy" "serverless_wedding_rsvp_handler_iam_policy" {
    name = "serverless-wedding-rsvp-handler-${var.environment_code}-iam-policy"
    role = "${aws_iam_role.serverless_wedding_rsvp_handler_iam_role.id}"

    policy = "${data.template_file.serverless_wedding_rsvp_handler_iam_policy.rendered}"
}

resource "aws_sns_topic" "rsvp_update_notifications" {
  name = "rsvp-updates-${var.environment_code}-sns-topic"
}