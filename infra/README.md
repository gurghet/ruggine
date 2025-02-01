# Infrastructure Configuration

This directory contains the Kubernetes infrastructure configuration for the Ruggine URL shortener service. The setup follows GitOps principles and uses Kustomize for environment-specific configurations.

## Directory Structure

```
infra/
├── base/                 # Base configuration shared across all environments
│   ├── deployment.yaml   # Main deployment, service, and ingress definitions
│   ├── namespace.yaml    # Namespace configuration
│   ├── doppler-secret.yaml # Doppler secret configuration
│   └── kustomization.yaml # Base kustomization file
└── overlays/            # Environment-specific configurations
    ├── staging/         # Staging environment
    │   ├── kustomization.yaml    # Staging kustomize config
    │   └── patch-deployment.yaml # Staging-specific patches
    └── prod/            # Production environment
        ├── kustomization.yaml    # Production kustomize config
        └── patch-deployment.yaml # Production-specific patches
```

## Base Configuration

The base configuration in `base/` defines the core components:
- Deployment with resource limits and security context
- Service exposing port 3000
- Ingress with TLS configuration via cert-manager
- Doppler secret for configuration management

## Environment Overlays

### Staging Environment
Located in `overlays/staging/`:
- Uses staging.codecraft.engineering domain
- Runs 2 replicas for redundancy
- Uses staging-specific labels for isolation
- Maintains separate TLS certificate

### Production Environment
Located in `overlays/prod/`:
- Uses codecraft.engineering domain
- Follows stable release process
- Production-grade configuration
- Separate TLS certificate management

## GitOps Workflow

1. Base configuration provides the template
2. Environment overlays patch the base configuration
3. Changes are made through git commits
4. GitOps operator (e.g., Flux/ArgoCD) applies changes
5. No direct kubectl commands needed

## Usage

- Staging automatically follows master branch
- Production updates only after successful E2E tests
- All changes must follow GitOps principles
- Use Kustomize for environment-specific changes:
  ```bash
  # View staging manifests
  kustomize build overlays/staging
  
  # View production manifests
  kustomize build overlays/prod
  ```
