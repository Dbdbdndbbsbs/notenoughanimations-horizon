// NEA MapRendering: a filled map held in a player's or mannequin's third-person
// hand is drawn by `notenoughanimations_held_map`, so the item model resolver
// returns an empty state for it and the flat icon is not drawn.
fn notenoughanimations_hide_hand_map(
    game: &crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
    display: horizon_assets::item_model::DisplayContext,
    owner: Option<&horizon_assets::item_model::OwnerState>,
) -> core::ops::ControlFlow<ItemRenderState> {
    use crate::game::client_ambient::native_nea_glue as glue;
    if !glue::settings(game).maps {
        return core::ops::ControlFlow::Continue(());
    }
    let third_person = matches!(
        display,
        horizon_assets::item_model::DisplayContext::ThirdPersonLeftHand
            | horizon_assets::item_model::DisplayContext::ThirdPersonRightHand
    );
    let avatar = owner.is_some_and(|o| {
        o.entity_type == "minecraft:player" || o.entity_type == "minecraft:mannequin"
    });
    // An empty map stays vanilla: never hide its flat icon.
    if &*stack.item == "minecraft:map" {
        return core::ops::ControlFlow::Continue(());
    }
    if third_person && avatar && stack.data().and_then(|d| d.map_id).is_some() {
        return core::ops::ControlFlow::Break(ItemRenderState {
            display,
            layers: Vec::new(),
            oversized_in_gui: false,
            animated: false,
            cit_glint: None,
            hdr_tier: 0,
            hdr_light: 0,
        });
    }
    core::ops::ControlFlow::Continue(())
}
