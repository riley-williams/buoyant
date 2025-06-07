cargo clean &&
  cargo build --all --all-features &&
  mdbook test -L target/debug/deps docs/book
