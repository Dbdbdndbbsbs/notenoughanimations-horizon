// Not Enough Animations' game half: the mod's row in Options > Mods and its
// one setting. `runtime.rs` holds the setting; the animation itself is a
// source patch in the game (mobs/anim.rs, ArmPose::Eat).

use std::cell::RefCell;

use crate::game::client_ambient::native_nea_runtime as m;
use crate::game::Game;

struct State {
    settings: Option<m::Settings>,
}

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| {
        f(cell
            .borrow_mut()
            .get_or_insert_with(|| State { settings: None }))
    })
}

pub(crate) fn settings(game: &Game<'_>) -> m::Settings {
    with(|s| {
        *s.settings
            .get_or_insert_with(|| m::Settings::load(|key| game.native_setting(m::ID, key)))
    })
}

pub(crate) fn click(game: &Game<'_>, button: usize) {
    let mut s = settings(game);
    s.click(button);
    for (key, value) in s.saved() {
        game.native_set_setting(m::ID, &key, &value);
    }
    with(|st| st.settings = Some(s));
}

/// The settings page's buttons: one per animation, named in the game's
/// language (the original mod's names).
pub(crate) fn labels(game: &Game<'_>) -> Vec<String> {
    let s = settings(game);
    let lang = &game.settings.language;
    s.labels(lang)
}

pub(crate) fn listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: Vec::new(),
    });
}
