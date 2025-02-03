# Ruggine URL Shortener

A URL shortener service built with Rust and deployed on Kubernetes.

## Features
- URL shortening
- Redirect service
- TLS support via cert-manager
- GitOps deployment with Flux
- Static file serving

## Environments
- Staging: https://staging.codecraft.engineering (uses Let's Encrypt staging certificates)
- Production: https://codecraft.engineering

## API Documentation

### Core Endpoints

#### `GET /`
- Description: Serves the main application page
- Response: HTML content
- Content-Type: `text/html; charset=utf-8`

#### `GET /:code`
- Description: Redirects to the original URL associated with the shortcode
- Parameters:
  - `code`: The shortcode (e.g., "B5Z")
- Response:
  - Success (307): Redirects to the target URL
  - Error (404): Returns "URL shortcode not found"
- Example: `GET /B5Z` redirects to https://codecraft.engineering

### Static Files

#### `GET /static/*`
- Description: Serves static PNG files from the static directory
- Parameters:
  - `*`: The file path relative to the static directory
- Response:
  - Success (200): Returns the file content
  - Error (404): Returns "File not found"
- Content-Type: `image/png` for successful responses
- Example: `GET /static/CodeCraft%20Engineering%20logo.png`

### System Endpoints

#### `GET /version`
- Description: Returns the application version
- Response: Version string
- Content-Type: `text/plain; charset=utf-8`

#### `GET /healthz`
- Description: Health check endpoint for Kubernetes
- Response: "OK" if the service is healthy
- Content-Type: `text/plain; charset=utf-8`

## Error Handling

All error responses follow a consistent format:
- 404 Not Found: Plain text error message with `text/plain; charset=utf-8` content type
- 400 Bad Request: Plain text error message with `text/plain; charset=utf-8` content type

## Security Notes

- Directory traversal attempts are blocked
- Static file serving is limited to PNG files
- Static directory listing is disabled
- URL-encoded paths are properly handled and sanitized

## Development

The service is built using:
- Axum web framework
- SQLite for URL storage
- Tower for middleware
- Base32 for shortcode generation
