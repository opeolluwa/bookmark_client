use bookmark_components::js_bindings::navigate::change_location_to;
use bookmark_state::{access_token::AccessToken, cached_user::CachedUser};

pub fn logout_current_user() {
    CachedUser::clear_user();
    AccessToken::set_empty();
    change_location_to("/");
}
