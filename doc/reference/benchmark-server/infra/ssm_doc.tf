resource "aws_cloudwatch_log_group" "deployer" {
  name              = "/${local.app_name}/deployer"
  retention_in_days = 180
}

resource "aws_ssm_document" "deployer" {
  name            = "${local.app_name}-deployer"
  document_type   = "Session"
  document_format = "JSON"

  content = templatefile("template/deployer_content.json", {
    cloud_watch_log_group_name: aws_cloudwatch_log_group.deployer.name
  })
}
