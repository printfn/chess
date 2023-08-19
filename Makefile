.PHONY: web fix

web:
	cd wasm && wasm-pack build --release --target web
	cd web && npm ci && npm run build

fix:
	cargo fmt
	cargo clippy -- -D warnings
	cd web && yarn format
	cd web && yarn lint
