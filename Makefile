make:
	cargo build --target wasm32-wasip1 --release
	func-e run -c envoy.yaml

push:
	cargo build --target wasm32-wasip1 --release
	docker build -t ghcr.io/konstfish/oauth-redirect-wasm:0.1.0 .
	docker push ghcr.io/konstfish/oauth-redirect-wasm:0.1.0