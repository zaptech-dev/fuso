# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in Fuso, please report it through GitHub's [private vulnerability reporting](https://github.com/zaptech-dev/fuso/security/advisories/new).

Do not open a public issue for security vulnerabilities.

We will acknowledge your report within 48 hours and aim to release a fix within 7 days of confirmation.

## Scope

Fuso is a local-only macOS app that reads a JSON config file from `~/.config/fuso/`. It makes no network requests and stores no credentials. Security concerns are primarily around config file parsing and local file access.
