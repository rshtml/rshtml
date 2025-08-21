test:
    echo "tests works.."
    cargo test --all-features --workspace

clippy:
    echo "clippy works.."
    cargo clippy --all-targets --all-features --workspace -- -D warnings

fmt:
    echo "fmt works.."
    cargo fmt --all --check

fix-fmt:
    echo "fixing formatting issues..."
    cargo fmt --all

build:
    echo "build works.."
    cargo build --release --workspace

doc:
    echo "doc works.."
    cargo doc --package rshtml --no-deps --open

publish_dry:
    echo "publish dry run works.."
    cargo publish --dry-run --package rshtml_core
    cargo publish --dry-run --package rshtml_macro
    cargo publish --dry-run --package rshtml

ci: test clippy fmt build publish_dry
    echo "CI works.."
