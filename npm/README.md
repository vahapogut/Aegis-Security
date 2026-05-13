# aegis-security

AI-era security scanner for LLM-generated code. Detects hallucinated vulnerabilities, auth bypasses, and AI-generated security anti-patterns in TypeScript, JavaScript, and Python before production.

## Quick Start

```bash
npm install -g aegis-security
aegis audit .
```

Or run without installing:

```bash
npx aegis-security audit .
```

## What It Detects

- **AI-auth-bypass** — JWT verified but no role/permission check
- **AI-floating-promise** — Unawaited async database/network calls
- **AI-silent-fail** — Catch blocks that swallow errors silently
- **AI-hardcoded-secret** — Placeholder API keys left in code
- **AI-hallucinated-import** — AI-generated imports for non-existent packages
- And 16 more AI-specific patterns...

## Formats

```bash
aegis audit .                    # colored terminal output
aegis audit . --format json      # machine-readable JSON
aegis audit . --format sarif     # GitHub Security tab integration
```

## Requirements

This npm package is a wrapper around the Aegis Rust binary. For the best experience, install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install aegis
```

## Links

- [GitHub Repository](https://github.com/vahapogut/Aegis-Security)
- [Full Documentation](https://github.com/vahapogut/Aegis-Security#readme)
- [IPEC Labs](https://ipeclabs.com)
