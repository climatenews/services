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

## Static Site Deployment (Cloudflare Pages)

The web frontend is built as a fully static site on the host (API/DB/cron stay
private) and published to Cloudflare Pages.

### Required env (see `static-publish.env` on the host)

- `GRAPHQL_API_URL` — private API URL used at build time (e.g. `http://localhost:8000/graphql`)
- `PUBLIC_DOMAIN` — site domain (default `climatenews.app`)
- `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID` — Cloudflare Pages credentials
- `CF_PAGES_PROJECT_NAME` (default `climatenews`), `CF_PAGES_BRANCH` (default `main`)

### Script

`devops/scripts/static-build-and-publish.sh`:

1. `git pull --ff-only` (unless `GIT_PULL=false`)
2. `npm ci`
3. `npm run build` — regenerates GraphQL codegen + sitemaps, then `next build && next export` → `web/out`
4. Fetches `newsFeedStatus.completedAt` from the private API and skips publish when unchanged (`SKIP_IF_UNCHANGED=true`)
5. `npx wrangler pages deploy web/out` when content changed

Runs are guarded by a lock file (`/tmp/climatenews-static-publish.lock`); logs
append to `APP_DIR/logs/static-publish.log`.

### systemd timer (host)

Units in `devops/systemd`:

- `climatenews-static-publish.service` (oneshot, reads `/etc/climatenews/static-publish.env`)
- `climatenews-static-publish.timer` (hourly)

Install:

```bash
sudo cp devops/systemd/climatenews-static-publish.{service,timer} /etc/systemd/system/
sudo install -d -m 0755 /etc/climatenews
sudo install -m 0640 /dev/null /etc/climatenews/static-publish.env
# populate /etc/climatenews/static-publish.env, then:
sudo systemctl daemon-reload
sudo systemctl enable --now climatenews-static-publish.timer
```

Run manually:

```bash
sudo systemctl start climatenews-static-publish.service
journalctl -u climatenews-static-publish.service -f
```

The old dynamic path (`docker-compose.prod.yaml`, `deploy-prod.sh`, Caddy
serving Next.js SSR) is being phased out in favor of this static export.

