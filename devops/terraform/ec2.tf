resource "aws_instance" "climate-news-service" {
  ami                    = var.aws_base_ami
  instance_type          = var.aws_instance_type
  key_name               = var.aws_key_name
  availability_zone      = var.aws_availability_zone
  vpc_security_group_ids = [aws_security_group.main.id]

  lifecycle {
    prevent_destroy = true
  }

  root_block_device {
    volume_size = 100
    volume_type = "gp3"
  }

  connection {
    type        = "ssh"
    user        = "ubuntu"
    private_key = file(var.aws_pem_file)
    host        = self.public_ip
  }

  # https://github.com/apriley/AWS-NiFiCluster-By-Terraform/blob/020ae8c285a9633aae23e72a2334703bd7553c75/DocumentStub/DocumentStub.tf
  provisioner "remote-exec" {
    inline = [
      "sudo apt-add-repository ppa:ansible/ansible -y",
      "sudo apt update",
      "sudo apt install ansible -y",
    ]
  }

}

resource "aws_eip" "elastic_ip" {
  instance = aws_instance.climate-news-service.id
  vpc      = true
}

# Security Group 
// TODO only allow ssh access from current ip
// Only allow traffic to post 443
resource "aws_security_group" "main" {
  egress = [
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = ""
      from_port        = 0
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "-1"
      security_groups  = []
      self             = false
      to_port          = 0
    }
  ]
  ingress = [
    {
      cidr_blocks      = ["38.13.80.132/32", ]
      description      = ""
      from_port        = 22
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 22
    },
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = ""
      from_port        = 443
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 443
    },
    {
      cidr_blocks      = ["0.0.0.0/0", ]
      description      = ""
      from_port        = 80
      ipv6_cidr_blocks = []
      prefix_list_ids  = []
      protocol         = "tcp"
      security_groups  = []
      self             = false
      to_port          = 80
    }
  ]
}

