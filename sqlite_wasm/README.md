# Sqlite Wasm

## Installation

```sh
cargo add sqlite_wasm
```

## Usage

- In your Tauri application

```rust
// src/lib.rs
let _ = SqliteWasm::init();
```

then in your handlers

```rust
use sqlite_wasm::models::user::UserInformation;

let _ = UserInformation::insert(...);
```
