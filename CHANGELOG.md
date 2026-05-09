# Changelog

## v0.1.0-alpha (2026-05-09)

### 🚀 Initial Release

**Aegis** — AI-era security scanner for LLM-generated code.

#### Core Engine
- Tree-sitter based AST analysis for TypeScript
- YAML-driven rule engine with precision-first philosophy
- Explanation Engine: every violation explains *why* the AI made the mistake
- AI Risk Score (0-100) for project-level risk assessment
- `.aegisignore` support for skipping directories

#### 10 Security Rules
| Rule | Description |
|------|-------------|
| `ai-auth-bypass` | JWT verified but no role/permission check |
| `ai-floating-promise` | Unawaited async database/network calls |
| `ai-silent-fail` | Catch blocks that swallow errors silently |
| `ai-hardcoded-secret` | Placeholder API keys left in code |
| `ai-regex-injection` | User input passed directly to `new RegExp()` |
| `ai-open-redirect` | User input controls redirect destination |
| `ai-insecure-fetch` | Disabled TLS/SSL verification |
| `ai-missing-rate-limit` | Sensitive routes without brute-force protection |
| `ai-unsafe-innerhtml` | innerHTML assignment without sanitization |
| `ai-fake-validation` | Validation checks that never halt execution |

#### Output Formats
- `--format text` — Colored terminal output (default)
- `--format json` — Machine-readable JSON with risk score

#### CI/CD
- GitHub Action (`action.yml`) for automated PR auditing
- Exit code 1 on HIGH violations for CI gate enforcement
