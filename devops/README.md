
# Terraform 
```bash
sudo terraform apply
tf apply -target=module.terraform_state_s3
```
# EC2 connect
```bash
# elastic ip
ssh -i "~/.ssh/ubuntu_desktop.pem" ubuntu@3.132.226.197

```


# Clone repo or download docker file?
## Add ssh key to github first
git clone git@github.com:climate-action/services.git 


# Ansible

```bash
ansible-playbook playbooks/docker.yml 
```

# Website
http://3.132.226.197:3000/


# Triggering a new Docker build
```bash
git tag -a v0.0.4 -m "compile fix"
git push origin v0.0.4 

```


