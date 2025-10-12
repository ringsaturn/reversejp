build:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --release --target web
	cp preview/index.html pkg/index.html

.PHONY: preview
preview:
	@echo "\033[1;32mPlease open http://localhost:9999\033[0m"
	cp preview/index.html pkg/index.html
	@cd pkg && python3 -m http.server 9999
