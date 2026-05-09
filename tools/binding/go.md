# tools/binding/go.md

# SpecFlow Binding Specification for Go

## 1. Overview

Go binding uses standard Go documentation comments (`//`).

Binding metadata is parsed from comment lines above:

- function
- method
- type
- package (doc.go)



## 2. Comment Format

Each binding line must follow:

```text
Key: Value
```

Example:

```go
// LoginUser authenticates user.
//
// Business: true
// Requirement: REQ-001
// Decision: ADR-003
// Flow: docs/05-flows.md#login-flow
// Purpose: Authenticate user using email and password.
// Layer: application
// Boundary: AuthModule
func LoginUser(ctx context.Context, req *LoginRequest) (*LoginResponse, error)
```



## 3. Supported Fields

### Classification (required if comment exists)

```text
Business: true
Support: true
Utility: true
```

Exactly one must be true.



### Business Binding Fields

Required for Business:

```text
Requirement: REQ-XXX [multiple allowed]
Purpose: ...
Layer: ui | api | application | domain | infrastructure
Boundary: ModuleName
Flow: docs/05-flows.md#flow-id
```

Optional:

```text
Story: US-XXX
Decision: ADR-XXX
Domain: DomainName
Errors: description
```



### Support / Utility Fields

```text
Support: true OR Utility: true
Purpose: ...
```

Must NOT include Requirement.



## 4. Package Binding (doc.go)

Each business package must include `doc.go`:

```go
// Package auth provides authentication services.
//
// Business: true
// Requirement: REQ-001
// Domain: Authentication
// Layer: application
// Boundary: AuthModule
package auth
```



## 5. Exported Symbols

Rules:

```text
Exported symbol without comment → warning
Exported symbol with comment → must follow SpecFlow rules
```



## 6. Private Functions

```text
No comment → allowed
Has comment → must include Business / Support / Utility
```



## 7. Multiple Values

Allowed:

```text
Requirement: REQ-001 REQ-002
Decision: ADR-001 ADR-002
```



## 8. Invalid Cases

```text
- Business=true but no Requirement
- Utility=true with Requirement
- Missing Purpose
- Missing Layer/Boundary for business
```



## 9. Parsing Notes

* Use `go/parser` and `ast`
* Read `CommentGroup`
* Ignore `_test.go`
* Ignore generated files

