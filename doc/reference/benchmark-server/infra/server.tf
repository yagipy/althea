data "aws_ami" "ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
  }

  filter {
    name = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["099720109477"]
}

resource "aws_instance" "api" {
  ami = data.aws_ami.ubuntu.id
  instance_type          = "t3.micro"
  iam_instance_profile = aws_iam_instance_profile.deployer.name
  subnet_id              = aws_subnet.public.id
  vpc_security_group_ids = [aws_security_group.allow_http.id]
}

resource "aws_eip" "api" {
  instance = aws_instance.api.id
  vpc = true
}

resource "aws_security_group" "allow_http" {
  name   = "allow-http"
  vpc_id = aws_vpc.this.id
  ingress {
    from_port       = 80
    to_port         = 80
    protocol        = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}
