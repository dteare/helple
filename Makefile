build:
	wasm-pack build --target web && cp pkg/helple_bg.wasm extension/.

run:
	cd www
	npm run start
