# Project Conventions

## Commit Message Standards

We follow Conventional Commits:

```
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

### Commit Message Rules:

- Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
- Subject capitalized, no period, ≤72 chars
- Body lines ≤72 chars
- Optional: Reference related issues in footer if applicable

## Public Interface Stability

- The public interface is defined by what is included in doctests
- Avoid changing public interfaces unless absolutely necessary
- Maintain backward compatibility
- Document breaking changes with "BREAKING CHANGE:" in footer or use "!" in subject
- Breaking changes require major version bump

## Linting & Testing

- Run tests and lints frequently, before and after every change
- Tests are best located as close as possible to the code they are testing.
- No `allow` attributes except for:
    ```rust
    #[allow(clippy::needless_pass_by_value)] // For quickcheck
    #[allow(clippy::multiple_crate_versions)] // Project-wide
    ```
- All mutation tests must pass
- Keep test coverage high (>90%)
- Warnings from clippy::nursery can be ignored

## Coding Principles

- Code should be easy to read and understand
- Keep code as simple as possible - avoid unnecessary complexity
- Use meaningful names that reveal intent
- Functions should:
    - Be small and do one thing well
    - Not exceed a few lines
    - Have descriptive names of the action being performed
    - Prefer fewer arguments (ideally ≤3)
- Comments should:
    - Only be used when necessary
    - Add useful information not apparent from the code
- Error handling:
    - Use proper error handling for robustness
    - Prefer exceptions over error codes
- Security:
    - Consider security implications
    - Implement security best practices

## Examples

Good:

```
feat(api)!: remove deprecated endpoints
```

```
docs(CONVENTIONS): Update conventions file

Add commit standards and interface stability rules.
```

Bad:

```
update conventions
```

## Functional Programming Principles

Adhere to these core FP principles:

1. Pure Functions - No side effects, same input → same output
2. Immutability - Avoid mutating state, prefer new values
3. Function Composition - Combine small functions into larger ones
4. Declarative Code - Focus on what to do, not how to do it

Avoid object-oriented programming patterns
