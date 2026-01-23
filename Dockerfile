FROM scratch
COPY target/wasm32-wasip1/release/oauth_redirect_wasm.wasm ./plugin.wasm

# https://github.com/istio-ecosystem/wasm-extensions/blob/master/doc/how-to-build-oci-images.md