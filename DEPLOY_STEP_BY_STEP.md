# 🚀 IA,BRO! Complete Deployment Guide - Step by Step

**Last Updated:** 2026-07-03  
**Status:** Production Ready  
**Deployment Time:** ~30 minutes

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Local Development](#local-development)
3. [Local Docker Testing](#local-docker-testing)
4. [Production Build](#production-build)
5. [Choose Your Platform](#choose-your-platform)
6. [Deployment Verification](#deployment-verification)
7. [Monitoring & Maintenance](#monitoring--maintenance)

---

## Prerequisites

### Required Tools:
- ✅ **Git** - `git --version`
- ✅ **Docker** - `docker --version` (v20.10+)
- ✅ **Docker Compose** - `docker compose version` (v2.0+)
- ✅ **Rust** (for local build) - `rustc --version` (1.75+)
- ✅ **Terminal/CLI** - Any shell (bash, zsh, fish, etc.)

### Optional But Recommended:
- GitHub CLI - `gh --version`
- jq - JSON processor
- curl - HTTP client

### Check Your Setup:
```bash
# Run this to verify everything
docker --version && docker compose version && git --version

# Expected Output:
# Docker version 20.10+
# Docker Compose version 2.0+
# git version 2.x+
```

---

## Step 1: Clone the Repository

```bash
# Clone your repo
git clone https://github.com/Vibecodingchiledev/IA-BRO-.git
cd IA-BRO-

# Verify structure
ls -la
# You should see: Cargo.toml, docker-compose.yml, Dockerfile, .env.example, etc.
```

---

## Step 2: Configure Environment

```bash
# Copy environment template
cp .env.example .env

# Edit with your credentials
nano .env  # or vim, code, etc.

# Required API Keys to add:
# GROQ_API_KEY=gsk_...
# ANTHROPIC_API_KEY=sk-ant-...
# OPENAI_API_KEY=sk-...

# Optional (for chat integrations):
# TELEGRAM_BOT_TOKEN=...
# DISCORD_BOT_TOKEN=...
# SLACK_BOT_TOKEN=xoxb-...
```

**Verify .env:**
```bash
# Check that .env has your keys
grep -E "GROQ|ANTHROPIC|OPENAI" .env

# Should show your API keys (not empty)
```

---

## Step 3: Local Development (Option A - Recommended for Testing)

### Build Locally with Rust:

```bash
# Install Rust dependencies
rustup update

# Build the workspace
cargo build --workspace --lib

# Expected time: 10-15 minutes (first time)
```

### Run Tests:

```bash
# Run all tests
cargo test --workspace

# Test the TUI specifically
cargo test -p ia-bro-tui

# Should see: test result: ok
```

### Run TODO TUI Locally:

```bash
# Terminal 1: Start the TUI
cargo run -p ia-bro-tui

# You should see the Terminal UI interface
# Create a task (press 'n', type, press Enter)
```

---

## Step 4: Local Docker Testing (Option B - Docker Compose)

### Build with Docker Compose:

```bash
# Build the Docker image (development mode)
docker compose build

# Expected output: Successfully built ...
# Time: 5-20 minutes depending on cache
```

### Verify Image:

```bash
# Check if image was created
docker images | grep ia-bro

# Should show your image with tag 'latest'
```

### Start the Application:

```bash
# Start in foreground (to see logs)
docker compose up

# Expected output:
# openfang-app  | Listening on 0.0.0.0:4200
# openfang-app  | Ready to accept connections

# Press Ctrl+C to stop
```

### Run in Background:

```bash
# Start in detached mode
docker compose up -d

# Check status
docker compose ps

# Expected output:
# NAME         STATUS      PORTS
# openfang-app running     0.0.0.0:4200->4200/tcp
```

### Test the API:

```bash
# In a new terminal, test the health endpoint
curl http://localhost:4200/api/health

# Expected response:
# {"status":"ok"}
```

### View Logs:

```bash
# Show logs from container
docker compose logs -f openfang

# Press Ctrl+C to exit logs
```

### Stop the Application:

```bash
# Stop services
docker compose down

# Stop and remove all data
docker compose down -v
```

---

## Step 5: Production Build

### Build Production Docker Image:

```bash
# Build optimized image for production
docker build -t ia-bro:latest \
  --build-arg LTO=true \
  --build-arg CODEGEN_UNITS=1 \
  .

# Expected output: Successfully built ...
# Time: 20-30 minutes (fully optimized)
```

### Verify Image Size:

```bash
# Check image size
docker images ia-bro

# Expected size: ~500MB - 1.5GB (depending on system)
```

### Test Production Image Locally:

```bash
# Run the production image
docker run -p 4200:4200 \
  --env-file .env \
  -v ia-bro-data:/data \
  ia-bro:latest

# Should start successfully
# Press Ctrl+C to stop
```

---

## Step 6: Push to GitHub Container Registry (GHCR)

### Authenticate with GHCR:

```bash
# Create Personal Access Token (PAT)
# Go to: https://github.com/settings/tokens/new
# Select scopes: write:packages, read:packages
# Copy the token

# Login to GHCR
echo "YOUR_PAT" | docker login ghcr.io -u USERNAME --password-stdin

# Expected: Login Succeeded
```

### Tag and Push Image:

```bash
# Tag the image for GHCR
docker tag ia-bro:latest ghcr.io/vibecodingchiledev/ia-bro-:latest

# Also tag with commit hash for versioning
docker tag ia-bro:latest ghcr.io/vibecodingchiledev/ia-bro-:$(git rev-parse --short HEAD)

# Push to GHCR
docker push ghcr.io/vibecodingchiledev/ia-bro-:latest
docker push ghcr.io/vibecodingchiledev/ia-bro-:$(git rev-parse --short HEAD)

# Expected: Pushed successfully
```

### Verify Upload:

```bash
# Check if image is on GHCR
curl -H "Authorization: Bearer YOUR_TOKEN" \
  https://ghcr.io/v2/vibecodingchiledev/ia-bro-/tags/list

# Should show: latest
```

---

## Step 7: Choose Your Deployment Platform

### **Option 1: Render.com (Easiest for Beginners)**

```bash
# 1. Go to https://render.com (sign up with GitHub)
# 2. Click "New +" → "Web Service"
# 3. Select GitHub repo (IA-BRO-)
# 4. Configure:
#    - Name: ia-bro
#    - Environment: Docker
#    - Region: (choose closest to you)
# 5. Add Environment Variables:
#    - GROQ_API_KEY = your_key
#    - ANTHROPIC_API_KEY = your_key
#    - RUST_LOG = info
# 6. Click "Create Web Service"
# 7. Wait 5-10 minutes for deployment

# Check status
echo "Your app is available at: https://ia-bro-*.onrender.com"
```

### **Option 2: Railway.app (Recommended)**

```bash
# 1. Install Railway CLI
npm install -g @railway/cli
# OR: curl -fsSL cli.new | bash

# 2. Login
railway login

# 3. Link project
cd IA-BRO-
railway link

# 4. Deploy
railway up

# 5. View deployment
railway status

# Your app URL will be shown
```

### **Option 3: Fly.io (Scalable)**

```bash
# 1. Install Fly CLI
curl -L https://fly.io/install.sh | sh

# 2. Login
fly auth login

# 3. Create app
fly launch --image ghcr.io/vibecodingchiledev/ia-bro-:latest

# 4. Set environment variables
fly secrets set GROQ_API_KEY=your_key ANTHROPIC_API_KEY=your_key

# 5. Deploy
fly deploy

# Check status
fly status
```

### **Option 4: DigitalOcean App Platform**

```bash
# 1. Go to https://cloud.digitalocean.com/apps
# 2. Click "Create App"
# 3. Select GitHub > IA-BRO- repo
# 4. Configure Container:
#    - Source: ghcr.io/vibecodingchiledev/ia-bro-:latest
#    - Port: 4200
# 5. Add Environment Variables
# 6. Choose plan and deploy

# Your app will be at: ia-bro-*.ondigitalocean.app
```

### **Option 5: Docker Compose on Your Server (Full Control)**

```bash
# On your server:
ssh user@your-server-ip

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Clone repo
git clone https://github.com/Vibecodingchiledev/IA-BRO-.git
cd IA-BRO-

# Configure
cp .env.example .env
nano .env  # Add your API keys

# Deploy
docker compose -f docker-compose.prod.yml up -d

# Verify
docker compose ps
curl http://localhost:4200/api/health
```

---

## Step 8: Deployment Verification

### Test Your Deployment:

```bash
# Replace YOUR_DOMAIN with your actual domain
YOUR_DOMAIN="ia-bro-*.onrender.com"  # or Railway/Fly/DO URL

# Test health endpoint
curl https://$YOUR_DOMAIN/api/health
# Expected: {"status":"ok"}

# Test in browser
open https://$YOUR_DOMAIN

# You should see the IA,BRO! frontend
```

### Check Logs:

**Render:**
```bash
# View in Render dashboard > Logs tab
# Or use curl
curl https://api.render.com/v1/services/YOUR_SERVICE_ID/events
```

**Railway:**
```bash
railway logs
```

**Fly.io:**
```bash
fly logs
```

**DigitalOcean:**
```bash
# View in dashboard > App > Logs
```

### Performance Test:

```bash
# Load test (requires Apache Bench)
ab -n 100 -c 10 https://$YOUR_DOMAIN/api/health

# Monitor response time
watch -n 1 'curl -w "\nTime: %{time_total}s\n" https://$YOUR_DOMAIN/api/health'
```

---

## Step 9: Monitor & Maintain

### Set Up Alerts:

```bash
# Render: Dashboard > Notifications
# Railway: Project > Alerts
# Fly: fly checks add
# DigitalOcean: App > Monitoring

# Key metrics to monitor:
# - Memory usage (< 500MB is good)
# - CPU usage (< 20% is good)
# - Uptime (target: 99.9%)
# - Response time (< 200ms is good)
```

### View Logs:

```bash
# Platform-specific log viewers:

# Railway
railway logs --tail 100

# Fly
fly logs -n 100

# Custom server
docker logs -f ia-bro-app --tail 100
```

### Database Backup:

```bash
# Backup tasks JSON
docker cp ia-bro-app:/data/tasks.json ./backup-$(date +%Y%m%d).json

# List backups
ls -lh backup-*.json
```

### Update Application:

```bash
# Pull latest code
git pull origin main

# Rebuild and deploy
docker compose down
docker compose up -d

# Verify
docker compose logs -f
```

---

## 🎯 Quick Reference Commands

### Local Development:
```bash
cargo run -p ia-bro-tui           # Run TODO TUI
cargo test --workspace             # Run tests
cargo build --release             # Build optimized binary
```

### Docker Commands:
```bash
docker compose up                  # Start dev
docker compose up -d              # Start in background
docker compose down               # Stop
docker compose logs -f            # View logs
docker build -t ia-bro:latest .   # Build image
docker run -p 4200:4200 ia-bro   # Run container
```

### Health Checks:
```bash
curl http://localhost:4200/api/health
curl https://your-deployment-url/api/health
docker exec ia-bro-app curl localhost:4200/api/health
```

---

## ✅ Deployment Checklist

- [ ] Prerequisites installed and verified
- [ ] `.env` file configured with API keys
- [ ] Local build successful (`cargo build`)
- [ ] Docker image builds without errors
- [ ] Container runs and responds to health check
- [ ] Image pushed to GHCR
- [ ] Platform account created
- [ ] Deployment secrets configured
- [ ] Application deployed successfully
- [ ] Health endpoint responding
- [ ] Logs visible and no errors
- [ ] Performance acceptable
- [ ] Monitoring configured
- [ ] Backup strategy in place

---

## 🆘 Troubleshooting

### Port Already in Use:
```bash
lsof -i :4200
kill -9 PID
```

### Docker Build Fails:
```bash
docker compose build --no-cache
docker system prune -a  # Clean docker
```

### Out of Memory:
```bash
# Increase Docker memory limit in:
# Docker Desktop > Settings > Resources > Memory
# Or in production, adjust container limits
```

### Can't Connect to Deployment:
```bash
# Check logs
docker logs ia-bro-app

# Verify network
docker network ls
docker inspect ia-bro-network
```

---

## 📞 Support

- **GitHub Issues:** https://github.com/Vibecodingchiledev/IA-BRO-/issues
- **Discussions:** https://github.com/Vibecodingchiledev/IA-BRO-/discussions
- **Documentation:** See `QUICKSTART.md` and `README.md`

---

**Happy Deploying!** 🚀

