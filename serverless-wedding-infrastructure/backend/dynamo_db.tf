
resource "aws_dynamodb_table" "rsvp_table" {
    name = "rsvp_table"
    read_capacity = 10
    write_capacity = 10
    hash_key = "household_id"
    range_key = "name"

    global_secondary_index {
        name               = "rsvp-id-index"
        hash_key           = "id"
        write_capacity     = 5
        read_capacity      = 5
        projection_type    = "ALL"
    }

    attribute {
        name = "household_id"
        type = "S"
    }

    attribute {
        name = "name"
        type = "S"
    }

    attribute {
        name = "id"
        type = "S"
    }
}
