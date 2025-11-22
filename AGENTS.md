# AGENTS

## Rules
- You are not allowed to use git commands other than the explicitly stated git commands bellow.
- You must comply with standarts for acceptance.
- You are not allowed to touch tests/ directory unless explicitly given permission to do so.
- You are not allowed to touch Traits deffinition unless explicitly allowed to. You must update documentation when changing these interfaces.
- User interface and config file must be described in README.md file.
- Traits must contain docstrings.


### Allowed git commands
- `git status`
- `git log`
- `git diff`

### Not allowed git commands
- `git add`
- `git commit`

### Standarts for acceptance
- `cargo check` compiles.
- `cargo test` has no failing tests.


### Testing guidelines
Never check for exact wording of error messages

## Documentation
-   [Project README](README.md)
-   [Detailed Documentation](doc/README.md)
