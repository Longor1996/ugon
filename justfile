default: run

binfeats := "--features 'cli'"

run *ARGS='':
    cargo run {{binfeats}} -- {{ARGS}}

docs:
    cargo doc --no-deps --open

check:
    cargo check {{binfeats}}

build:
    cargo build {{binfeats}} --release

build-dev:
    cargo build {{binfeats}}

build-docs:
    cargo doc --no-deps
