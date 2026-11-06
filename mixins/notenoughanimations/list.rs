// Not Enough Animations' row in Options > Mods.
fn notenoughanimations_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    crate::game::client_ambient::native_nea_glue::listed(mods);
}
