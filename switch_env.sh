#!/bin/bash

# Script to switch between different environments
# Usage: ./switch_env.sh [local|dev|staging|prod]

set -e

ENV=${1:-local}

case $ENV in
  local)
    ENV_FILE=".env.local"
    ;;
  dev|development)
    ENV_FILE=".env.development"
    ;;
  staging|stage)
    ENV_FILE=".env.staging"
    ;;
  prod|production)
    ENV_FILE=".env.production"
    ;;
  *)
    echo "❌ Unknown environment: $ENV"
    echo "Usage: $0 [local|dev|staging|prod]"
    exit 1
    ;;
esac

if [ ! -f "$ENV_FILE" ]; then
  echo "❌ Environment file not found: $ENV_FILE"
  exit 1
fi

# Backup current .env if it exists
if [ -f ".env" ]; then
  cp .env .env.backup
  echo "📦 Backed up current .env to .env.backup"
fi

# Copy the environment file
cp "$ENV_FILE" .env
echo "✅ Switched to $ENV environment"
echo "📄 Using configuration from: $ENV_FILE"
echo ""
echo "Current settings:"
grep -E "^(APP_ENV|PORT|RUST_LOG)" .env || true
echo ""
echo "🚀 You can now run: cargo run"
