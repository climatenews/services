# OpenTofu Deployment Map: AWS files to Hetzner resources

## Goal

Use the existing AWS Terraform files in `devops/terraform` as a reference map and deploy a new, lower-cost Hetzner environment with the same application topology:

- 1 VM host running Docker services (web, api, cron, postgres)
- public DNS for `climatenews.app`
- daily database backups to object storage

This guide assumes this is a new site (no production cutover required).

## AWS Reference Inventory

Reference resources are defined in:

- `devops/terraform/ec2.tf`
- `devops/terraform/route53.tf`
- `devops/terraform/s3_db_backup.tf`
- `devops/terraform/s3_terraform_state.tf`
- `devops/terraform/terraform.tf`
- `devops/terraform/variables.tf`

## AWS-to-Hetzner Resource Mapping

### Compute and Network

| AWS (current) | File | Hetzner target | OpenTofu provider/resource | Notes |
|---|---|---|---|---|
| `aws_instance.climate-news-service` | `ec2.tf` | 1 cloud server | `hcloud_server` | Direct replacement for single-host runtime. |
| `aws_eip.elastic_ip` | `ec2.tf` | static public IP | `hcloud_primary_ip` + attach to `hcloud_server` | Use a reserved primary IPv4 for stable A record. |
| `aws_security_group.main` | `ec2.tf` | host firewall rules | `hcloud_firewall` + `hcloud_firewall_attachment` | Mirror ports: 22 (restricted), 80, 443. |
| EC2 root volume (`root_block_device`) | `ec2.tf` | server disk snapshot policy | `hcloud_server` disk + `hcloud_snapshot` (optional) | Keep persistent postgres volume on host disk. |

### DNS

| AWS (current) | File | Hetzner target | OpenTofu provider/resource | Notes |
|---|---|---|---|---|
| `aws_route53_zone.production` | `route53.tf` | DNS zone | `hetznerdns_zone` (Hetzner DNS provider) | You can also use Cloudflare DNS if preferred. |
| `aws_route53_record.www` (A) | `route53.tf` | A record to VM IP | `hetznerdns_record` type A | Point apex to new primary IPv4. |
| `aws_route53_record.cname_www` | `route53.tf` | www alias | `hetznerdns_record` type CNAME | Keep `www -> apex`. |
| `aws_route53_record.mx` | `route53.tf` | MX | `hetznerdns_record` type MX | Preserve mail routing if you use the same email provider. |
| `aws_route53_record.txt` | `route53.tf` | TXT | `hetznerdns_record` type TXT | Preserve SPF and site verification entries. |

### Backups and Identity

| AWS (current) | File | Hetzner target | OpenTofu provider/resource | Notes |
|---|---|---|---|---|
| `aws_s3_bucket.db_backup` | `s3_db_backup.tf` | S3-compatible object storage | provider-specific bucket resource (for chosen storage) | Hetzner object storage or any S3-compatible low-cost bucket. |
| `aws_iam_policy`, `aws_iam_role`, `aws_iam_instance_profile` | `s3_db_backup.tf` | access keys/secrets for backup client | no Hetzner IAM equivalent | Replace instance profile flow with scoped object-storage credentials in env/secret file. |

### State Backend

| AWS (current) | File | Hetzner target | OpenTofu backend strategy | Notes |
|---|---|---|---|---|
| `backend "s3"` with AWS bucket + DynamoDB lock | `terraform.tf` | OpenTofu state backend | Option A: local state for first deploy (simplest). Option B: S3-compatible remote state for team use. | For a new site and solo workflow, local state is acceptable initially. |
| `aws_s3_bucket.terraform-state` + `aws_dynamodb_table.terraform-locks` | `s3_terraform_state.tf` | optional remote-state infrastructure | Provision only if you need shared state and locking | Not required to launch the new site. |

## OpenTofu Deployment Plan

### 1) Add Hetzner providers in a new OpenTofu root

Create a new folder (recommended): `devops/opentofu/hetzner`.

Use:

- `hetznercloud/hcloud` for servers, IPs, firewalls
- `timohirt/hetznerdns` (or equivalent) for DNS records

Keep AWS provider out of the new root unless you intentionally keep Route53 or S3 state there.

### 2) Translate variables

Map from current variables in `variables.tf`:

- `aws_instance_type` -> `hcloud_server_type` (example: `cpx21`)
- `aws_base_ami` -> `hcloud_image` (example: `ubuntu-24.04`)
- `aws_region` / `aws_availability_zone` -> `hcloud_location` (example: `hel1`, `nbg1`, `fsn1`, `ash`)
- `MY_IP_ADDRESS` stays, used in firewall rule for SSH
- `base_domain_name` stays unchanged

### 3) Reuse Ansible after host creation

Current Ansible flow in `devops/ansible/playbooks/main.yml` can remain mostly unchanged.

Update only:

- inventory host IP/name (`devops/ansible/production.yml`)
- any AWS-specific backup auth assumptions in backup scripts/environment

## OpenTofu Module Layout (recommended)

Use small modules to keep future cloud changes cheap:

- `modules/compute`:
  - server, primary IP, firewall, ssh key association
- `modules/dns`:
  - zone + records
- `modules/backup`:
  - object storage bucket resources (if provider supports it) or just outputs/secrets wiring

## New-Site Bring-Up Sequence

1. Create Hetzner VM, firewall, and static IP with OpenTofu.
2. Point DNS A record to the static IP.
3. Install Docker and deploy stack on the VM.
4. Initialize a fresh Postgres database.
5. Validate health endpoints (`/health`, GraphQL, web homepage).
6. Enable scheduled DB backups to object storage.
7. Monitor logs and cron behavior for the first 24-48h.

## What Can Stay As-Is

- App containers and Dockerfiles
- Scheduler behavior and cron cadence
- Ansible role logic for Docker installation

## What Must Change

- Terraform provider/resources (AWS to Hetzner)
- DNS provider resources if moving off Route53
- Backup credential model (IAM role to key-based credentials)
- Backup implementation (daily SQL dump to PITR-capable strategy)

## Database Backup Architecture (Recommended)

For a new single-VM production deployment, prefer a point-in-time recovery (PITR) capable setup over one daily logical dump.

### Backup Stack

- Backup tool: `pgBackRest` (or `WAL-G` as an alternative)
- Storage target: S3-compatible object storage (Hetzner Object Storage is suitable)
- Method: base backups + continuous WAL archiving

### Schedule

- Full backup: daily (off-peak)
- Incremental backup: every 6 hours
- WAL archive push: continuous
- Weekly restore verification: automated to disposable DB/container

### Retention Policy

- Daily backups: 7
- Weekly backups: 4
- Monthly backups: 6
- WAL retention aligned with your recovery point objective (RPO)

### Recovery Targets

- RPO target: <= 15 minutes
- RTO target: 30-90 minutes depending on backup size and VM class

### Security and Reliability

- Encrypt data in transit (TLS to object storage)
- Encrypt backup objects at rest
- Store object-storage credentials in root-only env file or secret manager
- Alert on backup failure and failed restore verification
- Run a restore drill at least weekly; backup jobs are only "trusted" if restores pass

### Operational Notes for This Repo

- Current script in `devops/cron/postgres-backup.sh` is a useful baseline but should be replaced for production.
- Keep logical dumps as optional secondary exports (for portability), not as the primary recovery strategy.
- Add a small runbook in docs with exact commands for:
  - latest full restore
  - point-in-time restore
  - restore verification workflow

## Cost-Oriented Defaults

For this repo, start with:

- one Hetzner VM with enough RAM for postgres + web + api + cron
- no load balancer
- PITR-capable backups (full + incremental + WAL) + weekly restore test
- keep DNS simple (apex + www + required MX/TXT)

## Risks and Mitigations

1. State management risk
   - Mitigation: use local state for initial launch, then migrate to remote backend once stable.
2. Undersized VM
   - Mitigation: start one tier above minimum if cron spikes; resize after metrics.
3. TLS/proxy misconfiguration
   - Mitigation: verify reverse proxy and certificates before opening public traffic.
4. Backups exist but cannot be restored
  - Mitigation: automate weekly restore verification and alerting.

## Execution Checklist

- [ ] Create new OpenTofu root for Hetzner
- [ ] Add Hetzner providers and variables
- [ ] Provision VM, firewall, primary IP
- [ ] Provision DNS zone/records
- [ ] Deploy app stack and run migrations
- [ ] Initialize fresh database (or restore seed backup if desired)
- [ ] Health checks pass (`/health`, web homepage, GraphQL endpoint)
- [ ] Configure pgBackRest/WAL-G with object storage credentials
- [ ] Enable full + incremental + WAL backup jobs
- [ ] Enable weekly restore verification job
- [ ] Add alerts for backup and restore failures
- [ ] Observe logs and cron jobs
