
resource "null_resource" "build_lambda" {

  provisioner "local-exec" {
    command = "cd ../src/idempotency_lambda"
  }

  provisioner "local-exec" {
    command = "cargo build --release --target x86_64-unknown-linux-musl"


  }
  provisioner "local-exec"{
    command = "cp target/x86_64-unknown-linux-musl/release/idempotency_lambda bootstrap"
  }

}

# IDP LAMBDA
data "archive_file" "zip" {
  depends_on = [null_resource.build_lambda]
  type = "zip"
  source_file = "../src/idempotency_lambda/bootstrap"
  output_path = "idp_lambda.zip"
}

resource "aws_iam_role" "lambda_execution_role" {
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

}

resource "aws_iam_policy" "readwrite-policy" {
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Action" : [
          "dynamodb:BatchGetItem",
          "dynamodb:Describe*",
          "dynamodb:List*",
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:Query",
          "dynamodb:Scan",
          "dynamodb:PartiQLSelect"

        ],
        "Effect" : "Allow",
        "Resource" : "*"
      },
      {
        "Action" : "cloudwatch:GetInsightRuleReport",
        "Effect" : "Allow",
        "Resource" : "arn:aws:cloudwatch:*:*:insight-rule/DynamoDBContributorInsights*"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "lambda_execution_policy" {
  role       = aws_iam_role.lambda_execution_role.name
  policy_arn = aws_iam_policy.readwrite-policy.arn
}

resource "aws_lambda_function" "idp_lambda" {
  function_name = "idp_lambda"

  source_code_hash = data.archive_file.zip.output_base64sha256
  filename         = data.archive_file.zip.output_path

  handler = "func"
  runtime = "provided"

  role = aws_iam_role.lambda_execution_role.arn
}
