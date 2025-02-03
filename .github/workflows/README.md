# GitHub Actions Workflows

This directory contains the CI/CD workflows for the Ruggine URL shortener service. The workflows implement a GitOps deployment strategy with staged environments and automated testing.

## Workflows

### 1. Build and Deploy Docker Image (`docker-deploy.yaml`)
Triggered on push to master branch and pull requests to master.

**Purpose:**
- Builds the Rust application
- Creates and pushes Docker image
- Updates staging environment

**Key Features:**
- Uses ARM64 runners (ubuntu-24.04-arm)
- Multi-stage Docker builds for optimization
- Automatic staging deployment via GitOps
- GitHub Container Registry integration
- Concurrency control to cancel in-progress builds

### 2. E2E Tests and Production Promotion (`e2e-and-promote.yaml`)
Triggered after successful `docker-deploy.yaml` completion on master branch.

**Purpose:**
- Tests staging environment
- Promotes to production if tests pass

**Process:**
1. Waits for staging to be ready
2. Runs E2E tests:
   - Valid URL redirect test
   - Invalid URL handling test
   - Static file serving test
   - Health check test
3. If tests pass:
   - Gets image tag from staging
   - Updates production configuration
   - Commits changes via GitOps

## Deployment Flow

```mermaid
graph TD
    A[Push to Master] --> B[Build & Deploy]
    B --> C[Update Staging]
    C --> D[Run E2E Tests]
    D -->|Pass| E[Promote to Prod]
    D -->|Fail| F[Stop]
```

## Infrastructure Integration

- Uses GitOps principles
- No direct cluster access
- All changes through git commits
- ARM64 optimized builds
- Separate staging/prod environments
- Automated staging to production promotion

## Usage

The workflows run automatically on:
- Push to master branch
- Pull requests to master (only build and test)

No manual intervention required unless:
- E2E tests fail
- Manual production rollback needed
- Infrastructure changes required

## Runner Requirements

All workflows use ARM64 runners (`ubuntu-24.04-arm`) for:
- Native ARM64 builds
- Better performance
- Reduced resource usage
- Consistent build environment
