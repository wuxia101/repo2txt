
# 🟨 tools/binding/typescript.md

# SpecFlow Binding Specification for TypeScript

## 1. Overview

TypeScript binding uses JSDoc comments:

```ts
/**
 * ...
 */
```

Bindings are parsed from tags or key-value lines.

---

## 2. Comment Format

```ts
/**
 * Authenticates user.
 *
 * Business: true
 * Requirement: REQ-001
 * Decision: ADR-003
 * Flow: login-flow
 * Purpose: Authenticate user via API.
 * Layer: application
 * Boundary: AuthModule
 */
export async function loginUser(req: LoginRequest): Promise<LoginResponse>
```

---

## 3. Alternative Tag Format

Also supported:

```ts
/**
 * @business true
 * @requirement REQ-001
 * @decision ADR-003
 * @flow login-flow
 * @purpose Authenticate user
 * @layer application
 * @boundary AuthModule
 */
```

---

## 4. Supported Fields

Same semantics as Go.

---

## 5. Export Rules

```text
export function → exported → warning if no comment
function → private → allowed without comment
```

---

## 6. React Component Binding

```ts
/**
 * Login form component.
 *
 * Business: true
 * Requirement: REQ-001
 * Story: US-001
 * Purpose: Render login UI.
 * Layer: ui
 * Boundary: AuthUI
 */
export function LoginForm() { ... }
```

---

## 7. Class Binding

```ts
/**
 * User service.
 *
 * Business: true
 * Requirement: REQ-002
 * Layer: application
 * Boundary: UserModule
 */
export class UserService {}
```

---

## 8. Parsing Notes

Recommended:

```text
TypeScript Compiler API
ts-morph
```

---

## 9. Invalid Cases

Same as Go.

---

## 10. Adapter Strategy

```text
specflow-ts → outputs BindingRecord JSON
```

CLI consumes normalized output.


