#!/bin/bash
set -euo pipefail

START_TIME=$SECONDS

BUILD_VERSION=$(git rev-parse --short HEAD)

echo "$(date --utc +%FT%TZ): Starting deployment – version $BUILD_VERSION"

if [ -f .env ]; then
    set -a
    source .env
    set +a
    echo "Loaded .env"
else
    echo ".env file not found!"
    exit 1
fi

git pull --ff-only

echo "$(date --utc +%FT%TZ): Checking frontend for changes..."

FRONTEND_HASH_FILE=".frontend-last-build-hash"

CURRENT_FRONTEND_HASH=$(git ls-files src/ | xargs git hash-object 2>/dev/null | git hash-object --stdin 2>/dev/null || echo "no-files")

if [ -f "$FRONTEND_HASH_FILE" ] && [ "$(cat "$FRONTEND_HASH_FILE")" = "$CURRENT_FRONTEND_HASH" ]; then
    echo "$(date --utc +%FT%TZ): ✅ No changes in src/. Skipping build."
else
    echo "$(date --utc +%FT%TZ): 🔨 Frontend files changed. Building..."

    npm ci --ignore-scripts
    npm run build

    # Save hash so we skip next time
    echo "$CURRENT_FRONTEND_HASH" > "$FRONTEND_HASH_FILE"
    
    echo "$(date --utc +%FT%TZ): ✅ Frontend build complete."
fi

echo "$(date --utc +%FT%TZ): Releasing new server version."

# Build new images / compose environment
docker compose build

# Get current container IDs for API and Web services
OLD_CONTAINERS=$(docker compose ps -q cracked-stack-rust-server)

# Scale up API service to two replicas
echo "⚡ Scaling up to 2 replicas..."
BUILD_VERSION=$BUILD_VERSION docker compose up -d --no-deps --scale cracked-stack-rust-server=2

echo "⏳ Waiting for new containers to be healthy..."
sleep 5  # or better: poll health via Caddy admin API or custom script

# Remove old ones
echo "🗑️  Removing old containers..."
if [ -n "$OLD_CONTAINERS" ]; then
  echo "Removing old containers..."
  docker container rm -f $OLD_CONTAINERS || true
fi

# Scale back to 1 (or keep 2 if you want replicas)
echo "📉 Scaling back to 1 replica..."
docker compose up -d --no-deps --scale cracked-stack-rust-server=1

echo "$(date --utc +%FT%TZ): 📦 Pre-deploy DB backup..."
bash "$(dirname "$0")/scripts/backup.sh"
echo "$(date --utc +%FT%TZ): ✅ Backup done — proceeding with schema migration."

echo "📋 Applying SurrealDB schema..."
if [ -f schema.surql ]; then
  cat schema.surql | docker exec -i surrealdb /surreal sql \
    --endpoint http://localhost:8001 \
    --user ${SURREALDB_USER} \
    --pass ${SURREALDB_PASS} \
    --ns ${SURREALDB_NS} \
    --db ${SURREALDB_DB}
  echo "✅ Schema applied"
else
  echo "⚠️  schema.surql not found — skipping"
fi

echo "📋 Applying SurrealDB seed file..."
if [ -f seed.surql ]; then
  cat seed.surql | docker exec -i surrealdb /surreal sql \
    --endpoint http://localhost:8001 \
    --user ${SURREALDB_USER} \
    --pass ${SURREALDB_PASS} \
    --ns ${SURREALDB_NS} \
    --db ${SURREALDB_DB}
  echo "✅ Seed file applied"
else
  echo "⚠️  seed.surql not found — skipping"
fi

# Need to restart nginx on new build because of symlinks, most likely
docker compose restart cracked-stack-nginx

ELAPSED=$((SECONDS - START_TIME))
MINUTES=$((ELAPSED / 60))
SECONDS_LEFT=$((ELAPSED % 60))

echo "$(date --utc +%FT%TZ): ✅ Deployment complete – version $BUILD_VERSION"
echo "⏱️  Total time: ${MINUTES}m ${SECONDS_LEFT}s"