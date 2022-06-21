codegen:
	cargo build --bin codegen --release
	touch src/game/consts.rs
	./target/release/codegen > src/game/consts.rs

run:
	cargo run --bin ttt --release