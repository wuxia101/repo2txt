# AGENTS.md

Version: 4.0.0

# SpecFlow Source-Bound Development Constitution

## 1. Core Principle

This system follows Source-Bound SpecFlow:

Intent documents define product truth.
Source code defines engineering truth.
Generated documents are projections of source code.

Business code must be traceably bound to intent documents.
The `specflow` CLI is the executable validator of this protocol. Agent output is not considered complete until `specflow validate` passes.

## 2. System Model

The system has three immutable layers:

### 2.1 Intent Layer (Human Owned)

```
docs/01-requirements.md
docs/02-user-stories.md
docs/03-architecture.md
docs/04-domain-model.md
docs/05-flows.md
decisions/ADR-XXX.md
```

Defines:

- product intent
- user stories
- architecture boundaries
- domain concepts
- business flows
- architectural decisions

These documents:

- no implementation details
- no code structure enforcement
- define direction and constraints only


### 2.2 Source Binding Layer (Agent Owned)

Source code is executable documentation.

Rules:

- Business logic must bind to intent (REQ / US / ADR / Flow)
- Utility/support code must be explicitly marked
- Code without binding is invalid only if it represents business behavior

Language-specific binding rules are defined separately:

```
tools/binding/go.md
tools/binding/rust.md
tools/binding/typescript.md
```

### 2.3 Generated Layer (Tool Generated Only)

Generated artifacts:

```
generated/06-api.md
generated/07-database.md
generated/08-ui-spec.md
generated/09-test-plan.md
generated/coverage-report.md
generated/sync-report.md
generated/trace-map.md
```

Rules:

- Generated only from source
- NEVER manually edited
- Fix source, not generated output

### 2.4 Doctor Binding Rule

Doctor does not require every private helper to have comments.

Rules:

- Exported functions/types without comments produce warnings.
- Any commented symbol must follow SpecFlow binding rules.
- Private symbols may be uncommented.
- Private commented symbols must declare one of:
  - Business: true
  - Utility: true
  - Support: true
- Business symbols must include:
  - Requirement
  - Purpose
  - Layer
  - Boundary
  - Flow or Domain
---

## 3. Core Invariant

The system must always satisfy:

```

Intent → Source → Generated

```

Violations:

- Source does not implement intent
- Generated does not reflect source
- Missing trace binding

All are blocking errors.


## 4. Project Structure (Protocol Level Only)

Only protocol-level directories are fixed:

```

project/
├── AGENTS.md
├── docs/
├── decisions/
├── tasks/
├── generated/
├── migrations/
├── deploy/
├── tools/

```

Application code layout:

- must follow language ecosystem conventions
- must NOT be forced into `src/` or `tests/`
- must be auto-discovered by agents


## 5. Agent Roles

### 5.1 Implementation Agent

Responsibilities:

- implement code from intent documents
- bind code to REQ / US / ADR / Flow
- respect architecture and boundaries
- regenerate generated docs

Must not:

- invent requirements
- modify intent documents
- edit generated docs
- write unbound business logic

---

### 5.2 Test Agent

Responsibilities:

- generate tests from source bindings
- verify behavior matches intent
- detect uncovered requirements

Must not:

- modify code to pass tests
- invent behavior not defined in intent

---

### 5.3 Review Agent

Responsibilities:

- verify source binds to intent
- verify generated docs reflect source
- detect orphan code and missing coverage
- enforce boundary rules

May block merge if:

- missing bindings
- coverage gaps
- boundary violations
- generated drift

---

## 6. Binding Rule (Abstract)

Every business artifact must bind to intent.

Minimum binding:

```

Requirement
Purpose
Flow or Domain

```

Optional:

```

Story
Decision
Boundary
Errors

```

Exact syntax is language-specific and defined outside this file.

---

## 7. Coverage Rule

The system must ensure:

- every REQ is implemented OR explicitly deferred
- every user story has UI or flow coverage
- every business function has binding
- no orphan code exists

Coverage report is generated automatically.

---

## 8. Boundary Rule

Each component must declare:

```

layer: ui | api | application | domain | infrastructure
boundary: ModuleName

```

Cross-boundary violations are forbidden unless justified by ADR.

---

## 9. Drift Resolution

Two types:

### Intent vs Source

- If intent is correct → fix code
- If intent is wrong → update intent first

### Source vs Generated

- regenerate
- fix generator if needed
- never edit generated docs


## 10. Integration Rule

Frontend must rely on generated contracts.

Flow:

```
Backend → Generated API → Client/Mock → UI → Test
```

Mock must be temporary and traceable.

## 11. SpecFlow CLI Tooling

This project uses the `specflow` CLI as the enforcement tool for Source-Bound SpecFlow.

All agents must use this tool to validate source bindings, traceability, coverage, and generated documents.

### Required Commands

```bash
specflow scan
specflow trace
specflow coverage
specflow generate
specflow doctor
specflow validate
```

### Command Responsibilities

* `specflow scan`
  Scans source code comments and generates `generated/bindings.json`.

* `specflow trace`
  Generates requirement-to-source trace maps.

* `specflow coverage`
  Checks whether requirements are covered by source bindings.

* `specflow generate`
  Generates engineering documents from source code and bindings.

* `specflow doctor`
  Diagnoses project structure, binding quality, and SpecFlow rule violations.

* `specflow validate`
  Runs the full validation pipeline.

### Agent Workflow

Before modifying code, agents must read:

```text
docs/01-requirements.md
docs/02-user-stories.md
docs/03-architecture.md
docs/04-domain-model.md
docs/05-flows.md
decisions/
tasks/
```

After modifying code, agents must run:

```bash
specflow validate
```

If validation fails, the agent must fix the source code, comments, bindings, or intent documents according to the failure type.

Generated files must not be manually edited.

### Binding Rule

Business code must bind to intent documents.

Minimum business binding:

```text
Business: true
Requirement: REQ-XXX
Purpose: ...
Layer: ...
Boundary: ...
Flow: ... 
```

Support or utility code may use:

```text
Support: true
Purpose: ...
```

or:

```text
Utility: true
Purpose: ...
```

Private helper functions may remain uncommented if they do not represent business behavior or public API.

### Validation Rule

A task is not complete until:

```bash
specflow validate
```

passes successfully.

Agents must include the validation result in their final response.



## 12. Forbidden Actions

- writing business code without binding
- editing generated documents manually
- hiding business logic in utility code
- violating architecture boundaries
- modifying intent as Implementation Agent

## 13. Core Formula

```

Intent Docs → Source Binding → Generated Docs → Coverage → Review → Runtime Feedback

```

## 14. Final Principle

Human defines intent.

Agent writes executable documentation.

Tools generate contracts.

Tests verify behavior.

Review enforces traceability.

Production feeds back reality.


