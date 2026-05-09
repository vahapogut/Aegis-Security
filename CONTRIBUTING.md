# Contributing to Aegis

We are thrilled that you want to help make Aegis the standard for AI-era secure coding! The most valuable contribution you can make is writing high-quality rules that detect new LLM behavioral anti-patterns.

## How to Create a New Rule

Aegis uses **Tree-sitter** for AST parsing and a custom YAML schema for rules. We prioritize **precision over recall**. A rule that catches 100% of bugs but has a 10% false-positive rate is **rejected**. We only want rules with near 0% false positives.

### 1. Identify an AI Anti-pattern
Find a specific logical error or security flaw that AI (Copilot, Cursor) frequently generates. For example: "AI forgets to await database calls."

### 2. Write the YAML Rule
Create a new file in `rules/<language>/your_rule_name.yaml`.

```yaml
rule:
  id: "ai-your-pattern-name"
  language: "typescript"
  confidence: "HIGH" # or MEDIUM
  message: "Short description of the bug."
  explanation: "Detailed explanation of why AI does this, why it matters, and how to fix it."
  query: >
    (call_expression
      function: (identifier) @func
      (#match? @func "^(dangerousFunc)$")
    )
```

### 3. Test Your Rule
Add a snippet of vulnerable code to `vulnerable-app/src/index.ts` (or create a new test file).

Run the auditor to ensure your rule catches it:
```bash
cargo run -- audit ./vulnerable-app
```

Ensure it does **not** flag legitimate code (no false positives).

### 4. Submit a Pull Request
- Create a branch (`feature/rule-name`).
- Push your rule and test case.
- Open a PR and describe the AI behavior you are targeting.

## Philosophy

Please do not submit rules that are:
- Generic SAST rules (e.g., standard SQL injection without an AI codegen context).
- "AI Detection" rules (e.g., trying to detect if AI wrote the code based on variable names).
- High false-positive rules.

Aegis is about **AI-specific failure patterns**. Think about *how the AI thinks* and where it cuts corners.
