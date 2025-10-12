build:
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --release --target web

.PHONY: preview
preview: build
	@echo "\033[1;32mPlease open http://localhost:9999\033[0m"
	make build
	rm -rf dist
	mkdir dist
	cp -r pkg dist/pkg
	cp preview/index.html dist/index.html
	@cd dist && python3 -m http.server 9999
