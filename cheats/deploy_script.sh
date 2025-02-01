#!/bin/bash
set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}🚀 Starting deployment process...${NC}"

# Get the current git SHA
GIT_SHA=$(git rev-parse --short HEAD)

# Build the Docker image locally
echo -e "${YELLOW}📦 Building Docker image...${NC}"
docker build -t ghcr.io/gurghet/ruggine:$GIT_SHA .
docker tag ghcr.io/gurghet/ruggine:$GIT_SHA ghcr.io/gurghet/ruggine:latest

# Push the images
echo -e "${YELLOW}⬆️  Pushing Docker images...${NC}"
docker push ghcr.io/gurghet/ruggine:$GIT_SHA
docker push ghcr.io/gurghet/ruggine:latest

# Wait for Flux to detect changes
echo -e "${YELLOW}⏳ Waiting for Flux to detect changes...${NC}"
flux reconcile source git flux-system
flux reconcile kustomization ruggine

# Wait for the deployment to roll out
echo -e "${YELLOW}🔄 Waiting for deployment to roll out...${NC}"
kubectl rollout status deployment/ruggine -n ruggine

echo -e "${GREEN}✅ Deployment complete!${NC}"

# Show the pod status
echo -e "${YELLOW}📊 Current pod status:${NC}"
kubectl get pods -n ruggine

# Test the endpoint
echo -e "${YELLOW}🔍 Testing the endpoint...${NC}"
curl -k https://ruggine.codecraft.engineering
