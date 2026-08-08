## DevOps Guide

### Scope

Active deployment path:

- OpenTofu infrastructure in `opentofu/hetzner`
- Ansible host bootstrap in `ansible`
- Docker Compose runtime using `docker-compose.prod.yaml`

### Provision Infrastructure (OpenTofu)

```bash
cd opentofu/hetzner
cp terraform.tfvars.example terraform.tfvars

export TF_VAR_hcloud_token="..."
export TF_VAR_hetzner_dns_api_token="..."

tofu init
tofu plan
tofu apply
```

### Configure Host (Ansible)

```bash
cd ansible
ansible-playbook playbooks/main.yml
```

### Required Host Files

- `/home/ubuntu/services` (repo clone path)
- `/home/ubuntu/.env.prod` (runtime env file)

### Production Operations

Installed to `/usr/local/bin` by Ansible:

- `deploy-prod.sh`: pulls and starts production services.
- `check-prod-health.sh`: validates app, API, cron, and DB health checks.
- `check-backup-timers.sh`: inspects timer state and recent backup logs.

Typical workflow:

```bash
sudo /usr/local/bin/deploy-prod.sh
sudo /usr/local/bin/check-prod-health.sh
sudo /usr/local/bin/check-backup-timers.sh
```

### Backup Automation

Backup scripts:

- `devops/cron/postgres-maintenance.sh`

Run manually on host:

```bash
sudo /usr/local/bin/postgres-maintenance.sh backup
sudo /usr/local/bin/postgres-maintenance.sh verify
```

Systemd units/timers:

- `climatenews-postgres-backup.timer` (daily)
- `climatenews-postgres-restore-verify.timer` (weekly)

Inspect on host:

```bash
systemctl list-timers --all | grep climatenews-postgres
systemctl status climatenews-postgres-backup.timer
systemctl status climatenews-postgres-restore-verify.timer
```

Optional immediate validation:

```bash
sudo RUN_NOW=true /usr/local/bin/check-backup-timers.sh
```

### Legacy AWS Terraform

The older AWS Terraform stack remains in `terraform` for historical reference only and is not part of the active deployment path.

