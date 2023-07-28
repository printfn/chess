.PHONY: web fix

web:
	cd wasm && wasm-pack build --release
	cd web && yarn && yarn build

fix:
	cargo fmt
	cargo clippy -- -D warnings
	cd web && yarn format
	cd web && yarn lint
