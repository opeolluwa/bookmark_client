run-app:
	cd app && npm run tauri dev

build-app:
	cd app && npm run tauri build 

lint: 
	lint-ui
	lint-tauri
	lint-backend

lint-ui:	
	cd app && npm run lint

lint-tauri:
	cd app/tauri && cargo fmt && cargo clipy --fix --allow-dirty

lint-backend: 
	cd backend && cargo fmt && cargo clipy --fix --allow-dirty