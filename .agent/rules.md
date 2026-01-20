# Project Rules: htup

## Architecture (Strict Clean Architecture)
This project strictly adheres to **Clean Architecture**.

### 1. Workspace Structure
- **`core/`**: Pure Rust library. Contains Domain, UseCase, and Infra. **NO UI logic.**
- **`tui/`**: Presentation layer. Only handles input/output. Depends on `core`.

### 2. Dependency Rule (The Onion)
Dependencies must ONLY point INWARD.
`Infra` -> `UseCase` -> `Domain`

### 3. Layer Definitions (`core`)
- **Domain (`core/src/domain`)**:
  - Contains **Entities** (Structs) and **Repository/Gateway Traits**.
  - **MUST NOT** have external dependencies (no `reqwest`, `std::fs` inside entities).
- **UseCase (`core/src/usecase`)**:
  - Contains Business Logic.
  - Depends **ONLY** on Domain Traits.
  - Dependencies are injected (DI) via `Arc<dyn Trait>` or Generics.
- **Infra (`core/src/infra`)**:
  - Concrete implementations of Traits (e.g., `FsRequestRepository`, `ReqwestHttpClient`).

### 4. Presentation (`tui`)
- **MVU / Controller Pattern**:
  - Translates User Events -> UseCase Calls.
  - Translates UseCase Results -> View Models.
  - NO Business Logic allowed here.

### 5. Testing
- **Testability First**: All Side Effects (IO, Network, Time) must be behind Traits.
- **Unit Tests**: Domain and UseCase must be fully unit-testable using Mocks.
