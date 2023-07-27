.PHONY: web

web:
	cd wasm && wasm-pack build --release
	cd web && yarn && yarn build
