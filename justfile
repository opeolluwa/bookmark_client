#!/bin/bash

alias w:= watch
alias b:= build
alias l:= lint
alias install := install-dependencies

set dotenv-required
set dotenv-load := true
set dotenv-path := "./.envrc"
set export :=  true


default: 
    @just --list --list-heading $'Available commands\n'

[doc('Install the application dependencies')]
install-dependencies: 
    echo "Installing dependencies"
    npm install --save 
    cd tauri && cargo build


[doc('Lint')]
lint:
    npm run lint
    cd tauri && cargo fmt && cargo clippy

[doc('Run the application in watch mode')]
[group('watch')]
watch target:
    #!/usr/bin/env sh
    if [ $target = "android" ]; then
        npm run tauri android dev
    elif [ $target = "ios" ]; then 
        npm run tauri ios dev 
    else
        npm run tauri dev
    fi

[doc('build the application ')]
[group('watch')]
build target:
    #!/usr/bin/env sh
    if [ $target = "android" ]; then
        npm run tauri android build --apk
    elif [ $target = "ios" ]; then 
        npm run tauri ios build --aab
    else
        npm run tauri bild 
    fi