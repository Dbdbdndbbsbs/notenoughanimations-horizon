// Suppresses the vanilla render of a hand's item when the original NEA
// OffhandHidingAnimation says it hides: the off hand's item while the main
// hand holds a bow or crossbow (mirrored for the main hand). A shield is never
// hidden.
fn notenoughanimations_hide_forced_offhand(
    stack: &crate::items::ItemStack,
    display: horizon_assets::item_model::DisplayContext,
    owner: Option<&horizon_assets::item_model::OwnerState>,
) -> core::ops::ControlFlow<ItemRenderState> {
    use crate::game::client_ambient::native_nea_glue as glue;
    if &*stack.item == "minecraft:shield" {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(owner) = owner else {
        return core::ops::ControlFlow::Continue(());
    };
    let offhand_display = if owner.main_arm_left {
        horizon_assets::item_model::DisplayContext::ThirdPersonRightHand
    } else {
        horizon_assets::item_model::DisplayContext::ThirdPersonLeftHand
    };
    let main_display = if owner.main_arm_left {
        horizon_assets::item_model::DisplayContext::ThirdPersonLeftHand
    } else {
        horizon_assets::item_model::DisplayContext::ThirdPersonRightHand
    };
    let hide = if display == offhand_display {
        glue::hide_offhand()
    } else if display == main_display {
        glue::hide_main()
    } else {
        false
    };
    if !hide {
        return core::ops::ControlFlow::Continue(());
    }
    core::ops::ControlFlow::Break(ItemRenderState {
        display,
        layers: Vec::new(),
        oversized_in_gui: false,
        animated: false,
        cit_glint: None,
        hdr_tier: 0,
        hdr_light: 0,
    })
}
