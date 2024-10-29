alias w:= watch
alias b:= build
alias l:= lint
alias install := install-dependencies
alias migrate := generate-migration

set dotenv-required
set dotenv-load := true
set dotenv-path := "./backend/.env"
set export :=  true

default: 
    @just --list --list-heading $'Available commands\n'

[doc('Install the application dependencies')]
install-dependencies: 
    @echo "Installing website dependencies"
    cd website && npm  install --save

    @echo "Installing app dependencies"
    cd app && npm install
    cd app/tauri && cargo build 

    @echo "Installing backend dependencies"
    cd backend && cargo build 

    @echo "Installing development dependencies"
    cargo install cargo-watch 
    cargo install sea-orm-cli

[doc('run the application in development mode')]
run-app:
    cd app && npm run tauri dev

[doc('run the backend in developent mode')]
run-backend:
    @echo DATABASE_URL=$DATABASE_URL
    cd backend && cargo watch -qcx run

lint-app:
    cd app && npm run lint
    cd app/tauri && cargo fmt && cargo fix --allow-dirty
lint-backend:
    cd backend && cargo fmt && cargo fix --allow-dirty
lint-website:
    cd website && npm run lint 
[group('lint')]
lint target:
    @echo formating {{target}}
    @just lint-{{target}}

generate-migration name:
    sea-orm-cli migrate --migration-dir database/migration generate {{name}}

[group('watch')]
watch-app:
    cd app && npm run tauri dev 
watch-backend: 
    cd backend && cargo watch -qcx run 
watch-website:
    cd app && npm run dev 
watch target:
    @echo watching {{target}}
    @just watch-{{target}}


[group('build')]
build-app:
    cd app && npm run tauri build
build-backend: 
    cd backend && cargo build 
build-website:
    cd app && npm run build
build target:
    @echo building {{target}}
    @just build-{{target}}