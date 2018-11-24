resource "aws_route53_zone" "blinging_love" {
  name = "blinging.love"
}

resource "aws_route53_record" "root" {
  zone_id = "${aws_route53_zone.blinging_love.zone_id}"
  name    = "${aws_route53_zone.blinging_love.name}"
  type    = "A"

  alias = {
    name                   = "${aws_cloudfront_distribution.blinging_love_distribution.domain_name}"
    zone_id                = "${aws_cloudfront_distribution.blinging_love_distribution.hosted_zone_id}"
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "www" {
  zone_id = "${aws_route53_zone.blinging_love.zone_id}"
  name    = "www"
  type    = "CNAME"
  ttl     = 5

  records = [
    "${aws_route53_record.root.name}",
  ]
}

resource "aws_acm_certificate" "ssl_cert" {
  domain_name       = "blinging.love"
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_route53_record" "ssl_cert_validation" {
  name    = "${aws_acm_certificate.ssl_cert.domain_validation_options.0.resource_record_name}"
  type    = "${aws_acm_certificate.ssl_cert.domain_validation_options.0.resource_record_type}"
  zone_id = "${aws_route53_zone.blinging_love.zone_id}"
  records = ["${aws_acm_certificate.ssl_cert.domain_validation_options.0.resource_record_value}"]
  ttl     = 60
}

resource "aws_acm_certificate_validation" "ssl_cert_validation" {
  certificate_arn         = "${aws_acm_certificate.ssl_cert.arn}"
  validation_record_fqdns = ["${aws_route53_record.ssl_cert_validation.fqdn}"]
}
