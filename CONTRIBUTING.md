# Contributing

Thank you for your interest in contributing to **usesend-rs**! Contributions of all kinds are welcome — bug reports, feature requests, documentation improvements, and code changes.

## Getting Started

1. Fork the repository and clone your fork:

   ```sh
   git clone https://github.com/<your-username>/usesend-rs.git
   cd usesend-rs
   ```

2. Make sure you have **Rust 1.85+** installed:

   ```sh
   rustup update stable
   ```

3. Verify the project builds and tests pass:

   ```sh
   cargo check --all
   cargo test --all
   ```

## Development Workflow

### Code Style

This project uses standard Rust formatting and linting:

```sh
cargo fmt --all          # Format code
cargo clippy --all --all-targets -- -D warnings  # Lint
```

Please ensure both pass before submitting a PR.

### Running Tests

```sh
cargo test --all
```

### Project Structure

| Crate | Description |
|-------|-------------|
| [`usesend`](./usesend) | High-level SDK with builder pattern |
| [`usesend-api`](./usesend-api) | Low-level typed HTTP client and request/response models |

## Submitting Changes

1. Create a feature branch from `master`:

   ```sh
   git checkout -b feat/my-feature
   ```

2. Make your changes and commit with a descriptive message following [Conventional Commits](https://www.conventionalcommits.org/):

   ```sh
   git commit -m "feat: add batch contact creation"
   ```

3. Push and open a Pull Request against `master`.

## Reporting Issues

- Use [GitHub Issues](https://github.com/AprilNEA/usesend-rs/issues) to report bugs or request features.
- Include Rust version (`rustc --version`), OS, and a minimal reproduction if possible.

## License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
