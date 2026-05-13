# Changelog

## v0.1.0 (2026-05-14)

### Major Features

**Aegis** — AI-era security scanner for LLM-generated code.

#### Core Engine
- Tree-sitter based AST analysis for TypeScript, JavaScript, and Python
- YAML-driven rule engine with precision-first philosophy
- Explanation Engine: every violation explains *why* the AI made the mistake
- AI Risk Score (0-100) for project-level risk assessment
- `.aegisignore` support for skipping directories

#### 33 Security Rules
| # | Rule | Language | What it detects |
|---|------|----------|-----------------|
| 1 | `ai-auth-bypass` | TypeScript | JWT verified but no role/permission check |
| 2 | `ai-floating-promise` | TypeScript | Unawaited async database/network calls |
| 3 | `ai-silent-fail` | TypeScript | Catch blocks that swallow errors silently |
| 4 | `ai-hardcoded-secret` | TypeScript | Placeholder API keys left in code |
| 5 | `ai-regex-injection` | TypeScript | User input passed directly to `new RegExp()` |
| 6 | `ai-open-redirect` | TypeScript | User input controls redirect destination |
| 7 | `ai-insecure-fetch` | TypeScript | Disabled TLS/SSL verification |
| 8 | `ai-missing-rate-limit` | TypeScript | Sensitive routes without brute-force protection |
| 9 | `ai-unsafe-innerhtml` | TypeScript | innerHTML assignment without sanitization |
| 10 | `ai-fake-validation` | TypeScript | Validation checks that never halt execution |
| 11 | `ai-hardcoded-secret-js` | JavaScript | Hardcoded credentials in JS code |
| 12 | `ai-unsafe-eval` | JavaScript | Unsafe eval()/Function() usage |
| 13 | `ai-no-csrf` | JavaScript | State-mutating routes without CSRF |
| 14 | `ai-weak-crypto` | JavaScript | MD5/SHA1/DES usage |
| 15 | `ai-missing-auth-check` | JavaScript | Routes without auth middleware |
| 16 | `ai-bare-except` | Python | Bare except swallowing all errors |
| 17 | `ai-hardcoded-secret-py` | Python | Credentials hardcoded in source |
| 18 | `ai-sql-format-string` | Python | SQL injection via f-strings/format() |
| 19 | `ai-debug-true` | Python | DEBUG=True in production settings |
| 20 | `ai-pickle-load` | Python | Unsafe pickle deserialization (RCE) |
| 21 | `ai-hallucinated-import` | Python | AI-hallucinated package imports |

#### Output Formats
- `--format text` — Colored terminal output (default)
- `--format json` — Machine-readable JSON with risk score
- `--format sarif` — SARIF v2.1.0 for GitHub Security tab integration

#### CI/CD
- GitHub Action (`action.yml`) for automated PR auditing
- SARIF upload to GitHub Security tab
- Exit code 1 on HIGH violations for CI gate enforcement

#### Testing
- Unit tests for engine, parser, and rule modules
- Criterion benchmarks for performance validation
- Test fixtures: `test-clean/` (zero false positives) and `vulnerable-app/` (targeted detections)

#### Distribution
- Cargo crate (crates.io): `cargo install aegis`
- npm package: `npm install -g aegis-security`
- GitHub Releases with prebuilt binaries

---

## v0.1.0-alpha (2026-05-09)

### Initial Release

- Core engine with Tree-sitter parsing
- 10 TypeScript rules + 5 Python rules
- Text and JSON output
- GitHub Action scaffold
