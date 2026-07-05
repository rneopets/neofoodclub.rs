default: lint

nightly_cargo := `rustup which --toolchain nightly cargo`
nightly_rustc := `rustup which --toolchain nightly rustc`
nightly_rustdoc := `rustup which --toolchain nightly rustdoc`

# Formatting
fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

# Linting
clippy:
  cargo clippy --workspace --all-features --locked -- -D warnings

# Includes tests/examples/benches; requires nightly for this repo.
clippy-all:
  CARGO_TARGET_DIR=target/nightly RUSTC={{nightly_rustc}} RUSTDOC={{nightly_rustdoc}} {{nightly_cargo}} clippy --workspace --all-targets --all-features --locked -- -D warnings

clippy-fix:
  cargo clippy --fix --allow-dirty --allow-staged --workspace --all-features --locked -- -D warnings

# Includes tests/examples/benches; requires nightly for this repo.
clippy-fix-all:
  CARGO_TARGET_DIR=target/nightly RUSTC={{nightly_rustc}} RUSTDOC={{nightly_rustdoc}} {{nightly_cargo}} clippy --fix --allow-dirty --allow-staged --workspace --all-targets --all-features --locked -- -D warnings

# Common workflows
lint: fmt clippy

fix: fmt clippy-fix

# Testing
# Includes integration tests; requires nightly for this repo.
test:
  CARGO_TARGET_DIR=target/nightly RUSTC={{nightly_rustc}} RUSTDOC={{nightly_rustdoc}} {{nightly_cargo}} test --locked

# Python bindings (crates/python) - built via maturin, not plain cargo
py-build:
  cd crates/python && uv run --with maturin maturin develop

py-build-release:
  cd crates/python && uv run --with maturin maturin develop --release

py-test:
  cd crates/python && uv run --with pytest pytest tests/

py-test-coverage:
  cd crates/python && uv run --with pytest --with coverage pytest --cov=neofoodclub --cov-report=term-missing tests/

py-fmt:
  cd crates/python && uv run --with ruff ruff format .

py-lint:
  cd crates/python && uv run --with ruff ruff check .

py-dist:
  cd crates/python && uv run --with maturin maturin build --release

py-publish:
  cd crates/python && uv run --with maturin maturin publish

# Full Python workflow: rebuild extension then run tests
py-test-all: py-build py-test
