use bookmark_components::forms::sign_up::SignUpFormData;

pub static SIGN_UP_COMMAND: &'static str = "sign_up";
#[tauri::command]
pub async fn sign_up(payload: SignUpFormData) {
    println!("the form data is {:#?}", payload);
}
