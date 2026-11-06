// The mod's settings page: one button per animation, on or off.
fn notenoughanimations_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::client_ambient::native_nea_runtime as m;
    if mod_id != m::ID {
        return;
    }
    buttons.extend(crate::game::client_ambient::native_nea_glue::labels(game));
}
