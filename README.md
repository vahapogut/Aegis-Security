<div align="center">
  <img src="/C:/Users/vahap/.gemini/antigravity/brain/dad3f73d-a75a-4946-9e0e-2123c5bd3b0d/aegis_logo_1778334727699.png" width="200" alt="Aegis Logo"/>
  <h1>🛡️ Aegis</h1>
  <p><strong>AI-era security scanner for LLM-generated code.</strong></p>
</div>

Aegis is a deterministic, fast, and precise security scanner designed specifically to catch "AI anti-patterns" and hallucinations in modern codebases. 

Unlike traditional SAST tools that look for generic vulnerabilities and produce noisy false positives, Aegis specifically models the **incorrect thinking patterns** of LLMs (Copilot, Cursor, DeepSeek). It doesn't just tell you there's a bug; it tells you *why* the AI made the mistake.

```bash
$ cargo install aegis
$ aegis audit .
```

---

## 💡 Why Aegis Exists

The rise of AI coding assistants has drastically increased development speed, but it has introduced a new class of bugs:
* **Hallucinated validations** that look correct but do nothing.
* **Floating promises** because the AI forgot to add `await` during refactoring.
* **Silent failures** where errors are caught and swallowed with a simple `console.log`.
* **Missing authorization** where JWTs are checked but user roles are ignored.

Traditional linters miss these because the syntax is perfectly valid. Aegis uses **Tree-sitter** and a custom YAML rule engine to catch these behavioral security flaws with surgical precision.

## 🚀 Demo

```bash
$ aegis audit .

[HIGH] Possible Auth Bypass. Sensitive route lacks authorization (role) checks. in auth.ts:35
    Explanation: AI often implements JWT verification but forgets to check if the user actually has the privileges to access the route. If this is an admin or destructive route, ensure you verify req.user.role or req.user.permissions.

Scan Summary
────────────
HIGH:   1
MEDIUM: 0
LOW:    0

AI Risk Score: 10/100
```

*Note: Aegis prioritizes precision over noisy detection. We aim for 0% false positives.*

---

## 🎯 Rule Showcase (TypeScript MVP)

Aegis ships with an ultra-high-quality rule set focusing on genuine AI pitfalls.

| Rule ID | Problem Detected | Why it happens |
|---------|------------------|----------------|
| `ai-auth-bypass` | JWT validated, but no Role/Permission check | AI writes the middleware but forgets the business logic. |
| `ai-floating-promise` | Unawaited async DB/Network calls | Very common during AI refactoring. Causes race conditions. |
| `ai-silent-fail` | `catch(e) { console.log(e) }` | AI is lazy with error handling. Swallows critical production crashes. |
| `ai-regex-injection` | `new RegExp(req.query.q)` | AI directly passes user input to RegExp, causing ReDoS attacks. |
| `ai-fake-validation` | `if (!user) { console.log("err") }` | AI forgets the `return` statement, allowing auth bypass. |

---

## ⚡ Performance

Rust and Tree-sitter make Aegis blazingly fast.

```text
Scanned 12,000 LOC in 0.08s
```
Aegis adds zero noticeable overhead to your CI/CD pipeline.

---

## 🥊 Why not Semgrep?

"Isn't this just Semgrep?" No. While we share architectural inspiration (AST + YAML rules), Aegis is solving a fundamentally different problem with a different philosophy.

| Feature | Semgrep | Aegis |
|---------|---------|-------|
| **Core Focus** | Generic SAST | AI behavioral security |
| **Target Audience** | Security Engineers | Developers using AI Assistants |
| **Vulnerability Type** | General vulnerabilities (SQLi, XSS) | LLM-specific failure patterns (Hallucinations, Lazy logic) |
| **Detection Strategy** | Broad detection | Precision-first AI-risk detection (Zero false-positive goal) |
| **Output Context** | Standard linting warnings | Explanation Engine (Why the AI made this mistake) |

---

## ⚙️ GitHub Action

Developers don't want to install CLI tools. Just drop Aegis into your CI/CD pipeline and it will automatically audit AI-generated code in every Pull Request.

```yaml
name: Aegis Security Audit
on: [push, pull_request]

jobs:
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: vahapogut/aegis-action@v1
        with:
          format: 'sarif' # or json/text
```

## 🔌 Enterprise Integration

Aegis supports JSON and SARIF output formats for easy integration into GitHub Security tab, Datadog, or custom dashboards.

```bash
aegis audit . --format json
```

## 📜 License
MIT License. Built for the community by IPEC Labs.
