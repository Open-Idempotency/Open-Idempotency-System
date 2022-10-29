variable "aws_account_id" {
  default = "048962136615"
}

variable "service_name" {
  type = string
  default = "idp"
}

variable "stage" {
  type = string
  default = "dev"
}

variable "region" {
  type = string
  default = "us-east-2"
}

variable "log_retention_in_days" {
  type    = number
  default = 30
}

variable "log_level" {
  type    = string
  default = "info"
}