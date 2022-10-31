resource "aws_dynamodb_table" "IDP" {

  name     = "IDP"
  billing_mode = "PAY_PER_REQUEST"

  attribute {
    name = "ms_name"
    type = "S"
  }

  attribute {
    name = "ulid"
    type = "S"
  }

  hash_key = "ms_name"
  range_key = "ulid"
}