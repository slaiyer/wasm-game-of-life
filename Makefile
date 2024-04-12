wasm-build:
	wasm-pack build

npm-init:
	npm init wasm-app www

npm-install:
	npm install --prefix www

npm-audit-fix:
	npm audit fix --prefix www

npm-run-start:
	npm run start --prefix www

test:
	wasm-pack test --firefox --headless
