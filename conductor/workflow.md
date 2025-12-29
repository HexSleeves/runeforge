# Runeforge Development Workflow

## Overview

This document defines the development workflow for implementing tracks in the Runeforge project. All contributors must follow these guidelines to ensure consistent, high-quality code.

---

## Test Coverage Requirements

**Required Test Coverage: >80%**

All new code must maintain or improve the overall test coverage:

- Run `cargo tarpaulin` or `cargo llvm-cov` to measure coverage
- Each crate should have >80% line coverage
- Critical algorithms (FOV, pathfinding) should have >90% coverage
- Document any intentional coverage gaps

---

## Commit Strategy

**Commit After: Each Task**

Changes should be committed after completing each individual task:

- Commit granularly to maintain clear history
- Each commit should represent a logical unit of work
- Write descriptive commit messages following Conventional Commits

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples:**
```
feat(fov): implement symmetric shadowcasting algorithm

Add symmetric shadowcasting FOV algorithm with fraction-based
precision for better visual consistency.

Closes #42
```

```
test(pathfinding): add edge case tests for A* algorithm

Add tests for diagonal movement, unreachable targets, and
zero-length paths.
```

---

## Task Summary Recording

**Method: Git Notes**

Task summaries are recorded using Git notes:

```bash
# After completing a task and committing
git notes add -m "Task: Implement symmetric shadowcasting FOV
- Added FovMap struct with visibility tracking
- Implemented recursive shadowcasting algorithm
- Added 8 unit tests covering edge cases
- All tests passing, coverage at 87%"
```

### Viewing Git Notes

```bash
# Show notes for current commit
git notes show

# Show notes in log
git log --show-notes

# Push notes to remote
git push origin refs/notes/*
```

---

## Development Workflow

### Phase Structure

Each track is divided into phases, and each phase contains multiple tasks:

1. **Planning Phase** — Define requirements and architecture
2. **Implementation Phase** — Write code and tests
3. **Testing Phase** — Verify functionality and performance
4. **Documentation Phase** — Update docs and examples

### Task Implementation Process

For each task:

1. **Read the task description** from `conductor/tracks/<track_id>/plan.md`
2. **Create a feature branch** (optional, for larger tasks)
3. **Implement the task** following code style guidelines
4. **Write tests** to verify functionality
5. **Run tests** and ensure they pass
6. **Check code quality**:
   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   ```
7. **Commit changes** with descriptive message
8. **Add git note** summarizing the task completion
9. **Update task status** in `plan.md` (mark as complete)

---

## Phase Completion Verification and Checkpointing Protocol

At the end of each phase, a manual verification checkpoint is required:

### Protocol Steps

1. **Automated Checks** — Run all automated tests and quality checks:
   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   cargo tarpaulin --out Stdout  # or cargo llvm-cov
   ```

2. **Manual Review** — The developer must manually verify:
   - All phase tasks are complete and marked in `plan.md`
   - Code follows style guidelines in `conductor/code_styleguides/rust.md`
   - Documentation is updated for new public APIs
   - Examples are updated if APIs changed
   - Test coverage meets >80% requirement
   - No known bugs or issues remain

3. **User Confirmation** — The AI agent will:
   - Present a summary of the phase completion
   - List all completed tasks
   - Show test results and coverage metrics
   - Request explicit user confirmation to proceed

4. **Checkpoint Commit** — Upon confirmation:
   - Create a checkpoint commit: `chore(conductor): complete phase <phase_name>`
   - Add detailed git note with phase summary
   - Update track metadata in `conductor/tracks/<track_id>/metadata.json`

### Example Phase Completion Task

In `plan.md`, each phase ends with:

```markdown
- [ ] Task: Conductor - User Manual Verification 'Implementation Phase' (Protocol in workflow.md)
```

This task cannot be marked complete until the user explicitly confirms the phase is ready to proceed.

---

## Quality Gates

### Before Committing

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted with `rustfmt`
- [ ] No Clippy warnings
- [ ] Documentation is updated

### Before Marking Task Complete

- [ ] Task requirements are fully met
- [ ] Tests cover new functionality
- [ ] Code follows style guidelines
- [ ] Git commit created
- [ ] Git note added

### Before Completing Phase

- [ ] All phase tasks are complete
- [ ] Test coverage >80%
- [ ] Documentation is complete
- [ ] Examples are updated
- [ ] User has confirmed phase completion

---

## Continuous Integration

### Local CI Checks

Run these commands before pushing:

```bash
# Format check
cargo fmt --all --check

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Tests
cargo test --all

# Documentation
cargo doc --all --no-deps

# Build all examples
cargo build --examples
```

### Automated CI (GitHub Actions)

The project uses GitHub Actions for CI:

- Run on every push and pull request
- Test on Linux, macOS, and Windows
- Check formatting, linting, and tests
- Measure code coverage
- Build documentation

---

## Summary

| Aspect | Standard |
|--------|----------|
| **Test Coverage** | >80% required |
| **Commit Frequency** | After each task |
| **Task Summary** | Git notes |
| **Phase Completion** | Manual verification required |
| **Quality Gates** | Format, lint, test before commit |
| **CI** | GitHub Actions on all platforms |
