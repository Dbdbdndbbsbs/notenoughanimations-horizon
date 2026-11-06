// A click on the mod's settings page.
fn notenoughanimations_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::client_ambient::native_nea_runtime as m;
    if mod_id != m::ID {
        return;
    }
    crate::game::client_ambient::native_nea_glue::click(game, button);
}
