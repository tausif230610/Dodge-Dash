cls
echo release-is-compiling
cargo b --release
cls
echo dev/debug-is-compiling
cargo b 
cls
echo web-dev-is-compiling
cargo +nightly build -Zbuild-std --target wasm32-unknown-emscripten
cls
echo web-release-is-compiling
cargo +nightly build --release -Zbuild-std --target wasm32-unknown-emscripten
cls 
echo full compile done
pause
