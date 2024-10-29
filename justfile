alias install := install-dependencies
alias build := build-app
alias lint := lint-all
alias dev := run-app 
alias web := run-website
alias backend:= run-backend 

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
    cd backend && cargo watch -qcx run

[doc('lint all the projects')]
lint-all:
    cd app && npm run lint
    cd backend && cargo fmt && cargo fix --allow-dirty
    cd app/tauri && cargo fmt && cargo fix --allow-dirty
    cd website && npm run lint 

[doc('build the desktop app')]
build-app:
    cd app && npm run tauri build 

[doc('run the website')]
run-website:
    cd website && npm run dev 