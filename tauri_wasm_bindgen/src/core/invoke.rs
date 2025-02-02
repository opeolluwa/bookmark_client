use serde::{de::DeserializeOwned, Serialize};

use crate::hooks::TauriCommandArgument;

/// invoke tauri command
/// this is based on the assumption that the command will return result
pub async fn invoke_command<Arg, T, E>(command: &str, argument: Option<Arg>) -> Result<T, E>
where
    Arg: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
    E: Serialize + DeserializeOwned,
{
    let val = if argument.is_some() {
        let payload = argument.unwrap();
        let payload_js_value =
            serde_wasm_bindgen::to_value(&TauriCommandArgument { payload }).unwrap();

        let command_result = wasm_bridge::invoke_tauri_command(command, payload_js_value).await;
        serde_wasm_bindgen::from_value::<T>(command_result).unwrap()
    } else {
        todo!()
    };

    Ok(val)
}

mod wasm_bridge {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"],js_name = invoke)]
        pub async fn invoke_tauri_command(cmd: &str, args: JsValue) -> JsValue;

        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
        pub async fn invoke_tauri_command_without_args(cmd: &str) -> JsValue;

        // #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
        // pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
    }
}
