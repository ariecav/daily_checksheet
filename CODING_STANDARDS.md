# Clean Architecture, SOLID, DRY, and Coding Standards - Daily Checksheet & Evaluation Test

This document outlines the architectural and coding standards applied to the **daily_checksheet** full-stack Rust application.

## 1. Core Principles

### SOLID Principles
- **Single Responsibility Principle (SRP):** Each module, struct, and function has a single, well-defined responsibility. For example, database operations are strictly separated from HTTP/UI handling, and evaluation logic is separate from persistence.
- **Open/Closed Principle (OCP):** Components are open for extension but closed for modification. Database abstraction layers allow switching from SQLite3 to SQL Server without modifying core business rules or handlers.
- **Liskov Substitution Principle (LSP):** Traits are designed such that any implementation of a repository or storage backend can be substituted seamlessly without breaking the system contract.
- **Interface Segregation Principle (ISP):** Traits (interfaces) are kept small and client-specific. Repositories only expose methods relevant to specific entities rather than a monolithic database interface.
- **Dependency Inversion Principle (DIP):** High-level modules (business logic, handlers) depend on abstractions (traits) rather than low-level details (concrete SQLite or SQL Server implementations).

### Clean Code & DRY (Don't Repeat Yourself)
- **DRY:** Common validation logic, error handling, JSON parsing, and response formatting are encapsulated in shared utility modules and helper functions.
- **Nesting Limit:** Maximum depth of `if`/`else` or match statements is strictly capped at **3 levels**. Deeply nested conditionals are refactored using early returns, guard clauses, or extraction into smaller helper functions.
- **File Length Limit:** Source code files are kept focused and modular, strictly capped under **800 lines** per file.
- **Naming & Readability:** Meaningful domain-specific naming conventions are used. Magic numbers and hardcoded strings are extracted into constants or configuration structures.

## 2. Architecture Overview (Full-Stack Rust)

The application is structured using a modular layered architecture:
1. **Domain / Entities:** Core data structures (`Question`, `Section`, `Evaluation`, `Submission`, `Result`).
2. **Repository / Persistence Layer:** Database abstraction layer using traits, with a concrete SQLite3 implementation (designed for easy migration to SQL Server).
3. **Service / Business Logic Layer:** Evaluation calculation, score computation, question validation, and section management.
4. **Presentation / Interface Layer:** Embedded web server or CLI/TUI interface serving a lightweight HTML/CSS/JS frontend (or full-stack Rust rendering) for taking evaluation tests and displaying results, compiled into a single standalone executable (`.exe`).
