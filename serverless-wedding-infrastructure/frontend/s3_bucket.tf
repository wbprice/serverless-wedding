data "template_file" "bucket_access_policy" {
  template = "${file("${path.module}/iam/policies/s3-bucket-access.json")}"

  vars {}
}

resource "aws_s3_bucket" "blinging_love_bucket" {
  bucket        = "blinging.love"
  acl           = "public-read"
  force_destroy = true

  website {
    index_document = "index.html"
    error_document = "index.html"
  }

  policy = "${data.template_file.bucket_access_policy.rendered}"
}
