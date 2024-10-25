 use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[tauri::command]
pub async fn sign_up(user: User) {
println!("{:?}", user);
}



#[tauri::command]
pub async fn sign_in() {

}


#[tauri::command]
pub async fn authenticate() {

}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[derive(TS)]
#[ts(export)]
pub struct User {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String
}