alias b := build
alias br := build-release
alias c := check
alias t := test
alias tc := test-crate
alias rc := run-cmd

build:
	cargo watch --clear --exec run

build-release: 
	cargo build --release

check: 
	cargo watch --clear --exec check

test: 
	cargo watch --clear --ignore dump --shell "cargo test -- --nocapture"

test-crate CRATE:
	RUST_LOG=debug cargo watch --clear --shell "cargo test {{CRATE}} -- --nocapture"

run:
    cargo watch --clear --exec run --shell "./target/debug/qc 2 3 4 :add . && ./target/debug/qc 4 7 9 :add 2 3 5 :mul 1 1 :sub 20 5 :div ."

run-cmd CMD:
	cargo watch --clear --ignore dump --ignore data --shell "cargo run -- {{CMD}}"

# clean up feature branch BRANCH
done BRANCH:
	git checkout master
	git diff --no-ext-diff --quiet --exit-code
	git pull --rebase origin master
	git diff --no-ext-diff --quiet --exit-code {{BRANCH}}
	git branch -D {{BRANCH}}

publish:
	cargo build
	cargo publish
