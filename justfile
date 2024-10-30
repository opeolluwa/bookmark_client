alias w:= watch
alias b:= build
alias l:= lint
alias install := install-dependencies
alias migrate := generate-migration
alias entities:= generate-entities

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


# Lint all parts of the project
lint-all:
    cd app && npm run lint --fix
    # cd website && npm run lint --fix
    cargo fmt && cargo fix --allow-dirty

# Lint specific parts of the project
lint-app:
    cd app && npm run lint
    cd app/tauri && cargo fmt && cargo fix --allow-dirty

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
watch-app:
    cd app && npm run tauri dev 
watch-backend: 
    @echo DATABASE_URL=$DATABASE_URL
    @echo PORT=$PORT
    @echo DATABASE_URL=$DATABASE_URL
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