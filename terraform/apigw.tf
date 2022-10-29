resource "aws_apigatewayv2_api" "unique_id_gw" {
  name          = "idp_gw"
  protocol_type = "HTTP"
}

resource "aws_iam_role" "apigw-role" {
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "apigateway.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

data "template_file" "gateway_policy" {
  template = file("policies/api-gateway-permission.json")
}

resource "aws_iam_policy" "api-policy" {
  policy = data.template_file.gateway_policy.rendered
}

resource "aws_iam_role_policy_attachment" "api_exec_role" {
  policy_arn = aws_iam_policy.api-policy.arn
  role       = aws_iam_role.apigw-role.name
}

// creating api gateway
resource "aws_apigatewayv2_integration" "api" {
  api_id             = aws_apigatewayv2_api.unique_id_gw.id

  integration_type   = "AWS_PROXY"
  integration_uri = aws_lambda_function.idp_lambda.invoke_arn
  integration_method = "POST"
  depends_on = [aws_iam_role_policy_attachment.api_exec_role]
}

resource "aws_apigatewayv2_route" "idp_route" {
  api_id    = aws_apigatewayv2_api.unique_id_gw.id
  route_key = "ANY /{proxy+}"
  target = "integrations/${aws_apigatewayv2_integration.api.id}"
}

resource "aws_apigatewayv2_deployment" "api" {
  api_id        = aws_apigatewayv2_api.unique_id_gw.id
  lifecycle {
    create_before_destroy = true
  }


  depends_on = [aws_apigatewayv2_route.idp_route]
}

resource "aws_cloudwatch_log_group" "api-gw" {
  retention_in_days = 30
}

resource "aws_apigatewayv2_stage" "api-gw_stage" {
  api_id      = aws_apigatewayv2_api.unique_id_gw.id
  name        = var.stage
  auto_deploy = true
}

