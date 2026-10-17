# Alloy examples agent guide

Use `examples-index.json` as the first lookup surface. It lists every Cargo example target with its
source path, summary, exact run command, and structured runtime requirements. Filter the `runtime`
object before selecting an example; do not assume network access, credentials, node binaries, or
hardware are available.

Source-of-truth order:

1. `Cargo.toml` and the package manifests define dependency and feature behavior.
2. `examples-index.json` maps tasks to runnable targets and prerequisites.
3. The referenced Rust source is authoritative for the API usage.
4. `README.md` is the human-oriented overview.

When adding or renaming an example:

1. Start the source file with a concise `//!` summary.
2. Add it to `README.md`.
3. Add only deterministic, offline examples to `scripts/runtime-examples.txt`.
4. Run `python3 scripts/generate-example-index.py` and commit the updated index.
5. Run `cargo check --workspace --examples --all-features --locked`.

Do not add shared public RPC endpoints or placeholder credentials. Use the helpers in
`example-support` so missing configuration produces an actionable error. Keep helper modules under
a named example directory so Cargo does not expose them as accidental example targets.
