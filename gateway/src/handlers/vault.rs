use super::super::api_response::{ApiResponse, IntoApiResponse, ResponseBody};
use super::super::helpers::jwt::JwtClaims;

pub async fn some_protected_resources(_: JwtClaims) -> ApiResponse<ResponseBody<&'static str>> {
    Ok(ApiResponse::from_parts("Shhhhhhhh! Top secret!", None))
}
