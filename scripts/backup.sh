#!/bin/bash
# SurrealDB backup script for cracked-stack
# Usage: bash scripts/backup.sh
# Cron (daily 2 AM):  0 2 * * * /home/dev/Code/cracked-stack/scripts/backup.sh >> /var/log/surrealdb-backup.log 2>&1
set -euo pipefail

REPO_DIR="/home/dev/Code/cracked-stack"
BACKUP_DIR="/home/dev/backups/surrealdb"
RETAIN_DAYS=14
TIMESTAMP=$(date --utc +%Y%m%dT%H%M%SZ)
BACKUP_FILE="$BACKUP_DIR/backup-$TIMESTAMP.surql.gz"

# ---------------------------------------------------------------------------
# Remote rsync target — set BACKUP_RSYNC_DEST in .env or override here.
# Example:  user@backup-server:/backups/cracked-stack/surrealdb
# Leave empty to skip remote copy.
# ---------------------------------------------------------------------------
BACKUP_RSYNC_DEST="${BACKUP_RSYNC_DEST:-}"

mkdir -p "$BACKUP_DIR"

# Load env vars (same pattern as deploy.sh)
if [ -f "$REPO_DIR/.env" ]; then
    set -a
    source "$REPO_DIR/.env"
    set +a
else
    echo "$(date --utc +%FT%TZ): ❌ .env not found at $REPO_DIR/.env" >&2
    exit 1
fi

echo "$(date --utc +%FT%TZ): Starting SurrealDB backup → $BACKUP_FILE"

docker exec surrealdb /surreal export \
    --endpoint http://localhost:8001 \
    --user "$SURREALDB_USER" \
    --pass "$SURREALDB_PASS" \
    --ns  "$SURREALDB_NS" \
    --db  "$SURREALDB_DB" \
    - | gzip > "$BACKUP_FILE"

SIZE=$(du -sh "$BACKUP_FILE" | cut -f1)
echo "$(date --utc +%FT%TZ): ✅ Backup complete — $SIZE written to $BACKUP_FILE"

# ---------------------------------------------------------------------------
# Remote copy via rsync
# ---------------------------------------------------------------------------
if [ -n "$BACKUP_RSYNC_DEST" ]; then
    echo "$(date --utc +%FT%TZ): 📡 Syncing to remote: $BACKUP_RSYNC_DEST"
    rsync -az --delete "$BACKUP_DIR/" "$BACKUP_RSYNC_DEST/"
    echo "$(date --utc +%FT%TZ): ✅ Remote sync complete"
else
    echo "$(date --utc +%FT%TZ): ⚠️  BACKUP_RSYNC_DEST not set — skipping remote copy"
fi

# ---------------------------------------------------------------------------
# Prune local backups older than RETAIN_DAYS
# ---------------------------------------------------------------------------
PRUNED=$(find "$BACKUP_DIR" -name "backup-*.surql.gz" -mtime +"$RETAIN_DAYS" -print -delete | wc -l)
echo "$(date --utc +%FT%TZ): 🗑️  Pruned $PRUNED backup(s) older than $RETAIN_DAYS days"
