resource "aws_dynamodb_table" "IDP" {

  name     = "IDP"
  billing_mode = "PAY_PER_REQUEST"

  attribute {
    name = "ms-name"
    type = "S"
  }

  attribute {
    name = "ULID"
    type = "S"
  }

  hash_key = "ms-name"
  range_key = "ULID"
}