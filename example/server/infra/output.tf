output "instance_id" {
  value = aws_instance.api.id
}

output "public_ip" {
  value = aws_eip.api.public_ip
}
