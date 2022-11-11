# Use docker as non-root user
sudo groupadd docker
sudo usermod -aG docker ${USER}
su -s ${USER}


# EC2 connect
ssh -i "~/.ssh/ubuntu_desktop.pem" ubuntu@ec2-18-117-177-130.us-east-2.compute.amazonaws.com

# Terraform 
sudo terraform apply