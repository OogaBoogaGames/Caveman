all: workspace/build flint/build

all-release: workspace/build-release flint/build-release

clean: workspace/clean flint/clean

workspace/build:
	cargo build

workspace/build-release:
	cargo build --release

workspace/clean:
	cargo clean

flint/build:
	cd flint; wasm-pack build --target web --out-name flint --out-dir dist

flint/build-release:
	cd flint; wasm-pack build --release --target web --out-name flint --out-dir dist

flint/clean:
	cd flint; cargo clean; rm -rf dist/

FORCE: ;