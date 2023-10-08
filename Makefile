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
	cd flint; wasm-pack build --target web --scope oogaboogagames --out-name flint

flint/build-release:
	cd flint; wasm-pack build --release --target web --scope oogaboogagames --out-name flint

flint/clean:
	cd flint; cargo clean; rm -rf dist/

FORCE: ;