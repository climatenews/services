### PostgreSQL maintenance script

Use `postgres-maintenance.sh` for both backup and restore verification.

```bash
chmod +x postgres-maintenance.sh

# Run backup
sudo ./postgres-maintenance.sh backup

# Run restore verification
sudo ./postgres-maintenance.sh verify
```