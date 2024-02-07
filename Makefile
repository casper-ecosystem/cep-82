build:
	cd marketplace && cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
	cd custodial && cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
	wasm-strip marketplace/target/wasm32-unknown-unknown/release/contract.wasm
	wasm-strip custodial/target/wasm32-unknown-unknown/release/contract.wasm

clean:
	cd marketplace && cargo clean
	cd custodial && cargo clean