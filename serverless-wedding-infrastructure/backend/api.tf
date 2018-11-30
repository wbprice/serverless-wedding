data "template_file" "serverless_wedding_api_assume_role_policy" {
    template = "${file("${path.module}/iam/policies/assume-role-policy.json")}"
}

resource "aws_iam_role" "serverless_wedding_api_iam_role" {
    name = "serverless-wedding-api-iam-role"

    assume_role_policy = "${data.template_file.serverless_wedding_api_assume_role_policy.rendered}"
}

data "template_file" "serverless_wedding_api_iam_policy" {
    template = "${file("${path.module}/iam/policies/api-policy.json")}"

    vars = {
        rsvp_table_arn = "${aws_dynamodb_table.rsvp_table.arn}"
    }
}

resource "aws_iam_role_policy" "serverless_wedding_api_iam_policy" {
    name = "serverless-wedding-api-iam-policy"
    role = "${aws_iam_role.serverless_wedding_api_iam_role.id}"

    policy = "${data.template_file.serverless_wedding_api_iam_policy.rendered}"
}