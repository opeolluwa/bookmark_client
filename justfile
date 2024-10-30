alias w:= watch
alias b:= build
alias l:= lint
alias install := install-dependencies

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

    @echo "Installing desktop dependencies"
    cd desktop && npm install
    cd desktop/tauri && cargo build 

    @echo "Installing backend dependencies"
    cd backend && cargo build 

    @echo "Installing development dependencies"
    cargo install cargo-watch 
    cargo install sea-orm-cli


# Lint all parts of the project
lint-all:
    cd desktop && npm run lint --fix
    # cd website && npm run lint --fix
    cargo fmt && cargo clippy

# Lint specific parts of the project
lint-desktop:
    cd desktop && npm run lint
    cd desktop/tauri && cargo fmt && cargo clippy

lint-backend:
    cd backend && cargo fmt && cargo fix --allow-dirty

lint-website:
    cd website && npm run lint 

# Grouped lint command for targeted linting
[group('lint')]
lint target:
    @echo formatting {{target}}
    @just lint-{{target}}



generate-migration name:
    sea-orm-cli migrate --migration-dir database/migration generate {{name}}
generate-entities: 
    sea-orm-cli generate entity -o database/entities

[group('watch')]
watch-desktop:
    cd desktop && npm run tauri dev 
watch-backend: 
    @echo DATABASE_URL=$DATABASE_URL
    @echo PORT=$PORT
    @echo DATABASE_URL=$DATABASE_URL
    cd backend && cargo watch -qcx run 
watch-website:
    cd desktop && npm run dev 
watch target:
    @echo watching {{target}}
    @just watch-{{target}}


[group('build')]
build-desktop:
    cd desktop && npm run tauri build
build-backend: 
    cd backend && cargo build 
build-website:
    cd desktop && npm run build
build target:
    @echo building {{target}}
    @just build-{{target}}


run-migration: 
    sea-orm-cli migrate -d database/migration up 

generate target:
    @just generate-{{target}}