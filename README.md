Installation
===
The following commands will install rust and wasm32.
```
pacman -S rustup
rustup install stable
rustup target add wasm32-unknown-unknown
```
Now we compile lib.rs to wasm
```
wasm-pack build --target web --out-name wasm --out-dir ./static
```
Now we can serve the index.html with any method we like (for example just start with IDEA)

