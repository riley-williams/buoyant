cargo clean &&
  cargo build --all-features &&
  mdbook test -L target/debug/deps docs/book
