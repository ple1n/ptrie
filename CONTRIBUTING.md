# 🧑‍💻 Contributing

The usual process to make a contribution is to:

1. Check for existing related [issues on GitHub](https://github.com/vemonet/ptrie/issues)
2. [Fork](https://github.com/vemonet/ptrie/fork) the repository and create a new branch
3. Make your changes
4. Make sure formatting, linting and tests passes.
5. Add tests if possible to cover the lines you added.
6. Commit, and send a Pull Request.

## 🛠️ Development

Install dependencies:

```bash
rustup update
rustup toolchain install nightly
rustup component add rustfmt clippy
cargo install cargo-tarpaulin git-cliff cargo-outdated
pipx install pre-commit
pre-commit install
```

### 🧪 Tests

Run tests:

```bash
cargo test
```

Tests with coverage:

```bash
cargo tarpaulin -p ptrie --doc --tests --out html
```

> Start web server for the cov report: `python -m http.server`

### 📚 Docs

Generate docs locally:

```bash
cargo doc --all --all-features
```

> Start web server for the generated docs: `python -m http.server --directory target/doc`

### ⏱️ Benchmark

Running benchmarks requires to enable rust nightly: `rustup default nightly`

```bash
cargo bench
```

## 🏷️ New release

Publishing artifacts will be done by the `build.yml` workflow, make sure you have set the following tokens as secrets for this repository: `CRATES_IO_TOKEN`, `CODECOV_TOKEN`

1. Make sure dependencies have been updated:

   ```bash
   cargo update
   cargo outdated
   ```

2. Bump the version in the `Cargo.toml` file, create a new tag with `git`, and update changelog using [`git-cliff`](https://git-cliff.org):

   ```bash
   git tag -a v0.5.0 -m "v0.5.0"
   git push origin v0.5.0
   git cliff -o CHANGELOG.md
   ```

3. Commit, and push. The `release.yml` workflow will automatically create the release on GitHub, and publish to crates.io.
