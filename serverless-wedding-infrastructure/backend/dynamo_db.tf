
resource "aws_dynamodb_table" "rsvp-table" {
    name = "rsvp_table"
    read_capacity = 10
    write_capacity = 10
    hash_key = "household_id"
    range_key = "name"

    attribute {
        name = "household_id"
        type = "S"
    }

    attribute {
        name = "name"
        type = "S"
    }
}
