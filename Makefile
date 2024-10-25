# Makefile

.PHONY: dev build export start tauri lint android desktop

dev:
	cd app && npm run tauri dev 

build:
	npm run build 

lint:
	npm run lint 

android:
	source ~/.bash_profile && npm run android 

desktop:
	npm run desktop 

build-apps:
	npm run tauri build 

# Makefile
