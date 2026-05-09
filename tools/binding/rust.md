
# 🟧 tools/binding/rust.md


# SpecFlow Binding Specification for Rust

## 1. Overview

Rust binding uses Rust doc comments:

```rust
/// ...
```

Bindings are parsed from attributes in doc comments.



## 2. Comment Format

```rust
/// Authenticates user.
///
/// Business: true
/// Requirement: REQ-001
/// Decision: ADR-003
/// Flow: login-flow
/// Purpose: Authenticate user using email and password.
/// Layer: application
/// Boundary: AuthModule
pub fn login_user(req: LoginRequest) -> Result<LoginResponse, Error>
```



## 3. Supported Fields

Same as Go.



## 4. Module Binding

```rust
/// Module for authentication.
///
/// Business: true
/// Requirement: REQ-001
/// Domain: Authentication
/// Layer: application
/// Boundary: AuthModule
mod auth;
```



## 5. Visibility Rules

```text
pub fn → treated as exported → warning if no comment
fn → private → same rules as Go
```



## 6. Enum / Struct Binding

```rust
/// User domain model.
///
/// Business: true
/// Requirement: REQ-002
/// Domain: User
/// Layer: domain
/// Boundary: UserModule
pub struct User { ... }
```



## 7. Parsing Notes

Recommended:

```text
syn crate
rustdoc JSON
```



## 8. Invalid Cases

Same as Go.



## 9. Adapter Strategy

If Go scanner cannot parse Rust:

```text
specflow-rust → outputs BindingRecord JSON
```

Main CLI normalizes output.
