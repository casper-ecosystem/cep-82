build:
	cd marketplace && cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
	cd custodial && cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
	cd session && cargo build --release --target wasm32-unknown-unknown --no-default-features
	wasm-strip marketplace/target/wasm32-unknown-unknown/release/contract.wasm
	wasm-strip custodial/target/wasm32-unknown-unknown/release/contract.wasm
	wasm-strip session/target/wasm32-unknown-unknown/release/session.wasm

clean:
	cd common && cargo clean
	cd marketplace && cargo clean
	cd custodial && cargo clean
	cd session && cargo clean