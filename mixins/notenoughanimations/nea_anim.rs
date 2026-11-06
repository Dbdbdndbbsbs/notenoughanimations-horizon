// NEA's animation half, inserted into `game/client_ambient.rs`: the settings
// and the ported animations. `setup`'s HEAD hook collects the player's state
// (`NeaState`); the model-visibility HEAD hook (right after the vanilla
// `humanoid` pose) applies the animations on top.
use crate::mobs::anim::{Anim, Poses, State};
use crate::mobs::Mob;
use horizon_core::geometry::trig::Mth;
use std::cell::Cell;

/// NEA's settings, switched each frame from the mod's settings.
#[derive(Clone, Copy, Debug)]
pub struct NeaSettings {
    pub master: bool,
    pub eat_drink: bool,
    pub ladder: bool,
    pub boat: bool,
    pub item_swap: bool,
    pub burning: bool,
    pub freezing: bool,
    pub crawling: bool,
    pub sleep: bool,
}
impl Default for NeaSettings {
    fn default() -> Self {
        Self {
            master: true,
            eat_drink: true,
            ladder: true,
            boat: true,
            item_swap: true,
            burning: true,
            freezing: true,
            crawling: true,
            sleep: true,
        }
    }
}
thread_local! {
    static NEA: Cell<NeaSettings> = const {
        Cell::new(NeaSettings {
            master: true,
            eat_drink: true,
            ladder: true,
            boat: true,
            item_swap: true,
            burning: true,
            freezing: true,
            crawling: true,
            sleep: true,
        })
    };
    /// NEA FreezingAnimation: the local player's box touches powder snow
    /// (computed by the frame hook).
    static POWDER_SNOW: Cell<bool> = const { Cell::new(false) };
    /// NEA BurningAnimation: the local player's remaining fire ticks (the
    /// model's own `fire_ticks` is only synced for remote players).
    static FIRE_TICKS: Cell<i32> = const { Cell::new(0) };
    /// The current player's animation state, captured by the `setup` HEAD hook.
    static STATE: Cell<NeaState> = const { Cell::new(NeaState::NONE) };
}
pub fn set_nea(s: NeaSettings) {
    NEA.with(|c| c.set(s));
}
pub fn set_powder_snow(on: bool) {
    POWDER_SNOW.with(|c| c.set(on));
}
pub fn set_fire_ticks(ticks: i32) {
    FIRE_TICKS.with(|c| c.set(ticks));
}
fn nea() -> NeaSettings {
    NEA.with(|c| c.get())
}
fn nea_on(flag: bool) -> bool {
    let n = nea();
    n.master && flag
}
fn in_powder_snow() -> bool {
    POWDER_SNOW.with(|c| c.get())
}
fn fire_ticks() -> i32 {
    FIRE_TICKS.with(|c| c.get())
}

/// Everything the animations need from the mob and the render state, captured
/// before the vanilla pose runs.
#[derive(Clone, Copy, Debug)]
pub struct NeaState {
    pub is_player: bool,
    pub partial: f32,
    pub pos: f32,
    pub y: f32,
    pub age: f32,
    pub swing: f32,
    pub using_item: i32,
    pub using_off_hand: bool,
    pub swap_at: f32,
    pub climbing: bool,
    pub boat: bool,
    pub boat_left: f32,
    pub boat_right: f32,
    pub swim: f32,
    pub swim_in_water: bool,
    pub main_arm_left: bool,
    pub swing_right_arm: Option<bool>,
    pub main_special: bool,
    pub off_special: bool,
    pub spear: bool,
    pub sleep_pose: bool,
    pub fire_resistance: bool,
}
impl NeaState {
    pub const NONE: Self = Self {
        is_player: false,
        partial: 0.0,
        pos: 0.0,
        y: 0.0,
        age: 0.0,
        swing: 0.0,
        using_item: 0,
        using_off_hand: false,
        swap_at: 0.0,
        climbing: false,
        boat: false,
        boat_left: 0.0,
        boat_right: 0.0,
        swim: 0.0,
        swim_in_water: false,
        main_arm_left: false,
        swing_right_arm: None,
        main_special: false,
        off_special: false,
        spear: false,
        sleep_pose: false,
        fire_resistance: false,
    };
    pub fn capture(m: &Mob, s: &State) -> Self {
        let spear = m
            .extra
            .main_hand
            .as_deref()
            .and_then(crate::spear::Kinetic::of)
            .is_some()
            || m
                .extra
                .off_hand
                .as_deref()
                .and_then(crate::spear::Kinetic::of)
                .is_some();
        let special = |item: Option<&str>| {
            matches!(
                item,
                Some("minecraft:bow")
                    | Some("minecraft:trident")
                    | Some("minecraft:spyglass")
                    | Some("minecraft:goat_horn")
                    | Some("minecraft:crossbow")
                    | Some("minecraft:brush")
            ) || m.extra.blocking
        };
        Self {
            is_player: matches!(m.kind.anim, Anim::Humanoid { zombie_arms: false }),
            partial: s.partial,
            pos: s.pos,
            y: m.position.y as f32,
            age: s.age,
            swing: s.swing,
            using_item: m.extra.using_item,
            using_off_hand: m.extra.using_off_hand,
            swap_at: m.extra.swap_at,
            climbing: m.climbing,
            boat: m.extra.boat,
            boat_left: m.extra.boat_left_smooth,
            boat_right: m.extra.boat_right_smooth,
            swim: m.extra.swim.amount(s.partial),
            swim_in_water: m.extra.swim.in_water,
            main_arm_left: m.main_arm_left(),
            swing_right_arm: m.extra.swing_right_arm,
            main_special: special(m.extra.main_hand.as_deref()),
            off_special: special(m.extra.off_hand.as_deref()),
            spear,
            sleep_pose: crate::mobs::villager::sleep_pose(m).is_some(),
            fire_resistance: m.has_effect("minecraft:fire_resistance"),
        }
    }
}
pub fn set_state(st: NeaState) {
    STATE.with(|c| c.set(st));
}
fn state() -> NeaState {
    STATE.with(|c| c.get())
}

fn rotlerp_rad(a: f32, from: f32, to: f32) -> f32 {
    let mut diff = to - from;
    while diff < -std::f32::consts::PI {
        diff += std::f32::consts::TAU;
    }
    while diff >= std::f32::consts::PI {
        diff -= std::f32::consts::TAU;
    }
    from + a * diff
}

/// Apply every ported animation to a player model, after the vanilla pose.
pub fn apply_all(p: &mut Poses) {
    let st = state();
    if !st.is_player {
        return;
    }
    apply_eat_drink(p, &st);
    apply_ladder(p, &st);
    apply_boat(p, &st);
    apply_crawling(p, &st);
    apply_item_swap(p, &st);
    apply_burning(p, &st);
    apply_freezing(p, &st);
    apply_sleep(p, &st);
}

/// NEA EatDrinkAnimation (priority 2500): the using arm lifts to the mouth
/// and chews.
fn apply_eat_drink(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().eat_drink) || st.using_item <= 0 {
        return;
    }
    let using_main = !st.using_off_hand;
    let using_off = st.using_off_hand;
    let (arm, using) = if using_main && !st.main_special {
        ("right_arm", true)
    } else if using_off && !st.off_special {
        ("left_arm", true)
    } else {
        return;
    };
    if !using {
        return;
    }
    let t = st.using_item as f32 + st.partial;
    let rise = (t / 5.0).clamp(0.0, 1.0);
    let smooth = rise * rise * (3.0 - 2.0 * rise);
    let phase = (t * 0.125).fract();
    let lift = (phase * std::f32::consts::TAU).msin().abs();
    let chew = lift * smooth;
    let low = -0.85;
    let mouth = -1.55;
    let right = arm == "right_arm";
    p.xrot(arm, low + (mouth - low) * smooth - chew * 0.20);
    p.yrot(arm, if right { -0.60 * smooth } else { 0.60 * smooth });
    p.zrot(
        arm,
        if right {
            0.15 * smooth + chew * 0.25
        } else {
            -(0.15 * smooth + chew * 0.25)
        },
    );
}

/// NEA LadderAnimation (priority 1000): fixed arm/leg cos swings, hands up at
/// -1.7, amplitude 0.35, phase = world Y * 2.
fn apply_ladder(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().ladder) || !st.climbing {
        return;
    }
    // The phase uses the world Y; the model can't see it, so the vanilla
    // climbing flag + the current height from limb motion stand in.
    let c = (st.y * 2.0).mcos();
    let v = -c * 0.35;
    p.xrot("right_arm", -1.7 + c * 0.35);
    p.xrot("left_arm", -1.7 - c * 0.35);
    p.yrot("right_arm", 0.0);
    p.yrot("left_arm", 0.0);
    p.zrot("right_arm", 0.0);
    p.zrot("left_arm", 0.0);
    p.xrot("right_leg", v);
    p.xrot("left_leg", -v);
    p.yrot("right_leg", 0.0);
    p.yrot("left_leg", 0.0);
    p.zrot("right_leg", 0.0);
    p.zrot("left_leg", 0.0);
}

/// NEA BoatAnimation (priority 1000): each oar sweeps with its own rowing
/// time; idle holds the arms still.
fn apply_boat(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().boat) || !st.boat {
        return;
    }
    let row = |t: f32| -1.1 - t.msin() * 0.3;
    p.xrot("right_arm", row(st.boat_right));
    p.yrot("right_arm", 0.2);
    p.zrot("right_arm", 0.3);
    p.xrot("left_arm", row(st.boat_left));
    p.yrot("left_arm", -0.2);
    p.zrot("left_arm", -0.3);
}

/// NEA CrawlingAnimation (priority 350): the swimming pose on land crawls.
fn apply_crawling(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().crawling) || st.swim_in_water {
        return;
    }
    let amount = st.swim;
    if amount <= 0.0 {
        return;
    }
    let speed_mul = 2.5;
    let step = (st.pos * speed_mul) % 26.0;
    let step2 = (step + 13.0) % 26.0;
    let right = st.swing_right_arm;
    let m_amount = if right == Some(true) { 0.0 } else { amount };
    let n_amount = if right == Some(false) { 0.0 } else { amount };
    let lerp = |a: f32, from: f32, to: f32| from + a * (to - from);
    let rotlerp = |a: f32, from: f32, to: f32| rotlerp_rad(a, from, to);
    let quadratic = |x: f32| -65.0 * x + x * x;
    let arm = 0.370_796_4;
    let leg = 0.15;
    let r = 0.333_333_34;
    let (rx, ry, rz) = {
        let a = p.get("right_arm");
        (a[3], a[4], a[5])
    };
    let (tx, ty, tz) = if step < 14.0 {
        let stroke = 1.870_796_4 * quadratic(step) / quadratic(14.0);
        (0.0, std::f32::consts::PI, std::f32::consts::PI - stroke)
    } else if step < 24.0 {
        let o = (step - 14.0) / 10.0;
        (
            -arm * o,
            std::f32::consts::PI,
            1.270_796_3 + 1.870_796_4 * o,
        )
    } else if step < 26.0 {
        let pp = (step - 24.0) / 2.0;
        (
            -arm + arm * pp,
            std::f32::consts::PI,
            std::f32::consts::PI,
        )
    } else {
        (rx, ry, rz)
    };
    p.xrot("right_arm", lerp(m_amount, rx, tx));
    p.yrot("right_arm", lerp(m_amount, ry, ty));
    p.zrot("right_arm", lerp(m_amount, rz, tz));
    let (lx, ly, lz) = {
        let a = p.get("left_arm");
        (a[3], a[4], a[5])
    };
    let (tx, ty, tz) = if step2 < 14.0 {
        let stroke = 1.870_796_4 * quadratic(step2) / quadratic(14.0);
        (0.0, std::f32::consts::PI, std::f32::consts::PI + stroke)
    } else if step2 < 24.0 {
        let o = (step2 - 14.0) / 10.0;
        (
            -arm * o,
            std::f32::consts::PI,
            5.012_389 - 1.870_796_4 * o,
        )
    } else if step2 < 26.0 {
        let pp = (step2 - 24.0) / 2.0;
        (
            -arm + arm * pp,
            std::f32::consts::PI,
            std::f32::consts::PI,
        )
    } else {
        (lx, ly, lz)
    };
    p.xrot("left_arm", rotlerp(n_amount, lx, tx));
    p.yrot("left_arm", rotlerp(n_amount, ly, ty));
    p.zrot("left_arm", rotlerp(n_amount, lz, tz));
    let tc = st.pos * speed_mul;
    let x = p.get("left_leg")[3];
    p.xrot("left_leg", lerp(amount, x, leg * (tc * r + std::f32::consts::PI).mcos()));
    p.zrot("left_leg", -0.150_796_4);
    let x = p.get("right_leg")[3];
    p.xrot("right_leg", lerp(amount, x, leg * (tc * r).mcos()));
    p.zrot("right_leg", 0.150_796_4);
}

/// NEA ItemSwapAnimation (priority 3500): the arms cross as the hands trade
/// their items and unroll over 10 ticks.
fn apply_item_swap(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().item_swap) || st.swap_at <= 0.0 {
        return;
    }
    let elapsed = (st.age - st.swap_at) / 10.0;
    if (0.0..=1.0).contains(&elapsed) {
        let roll = -1.0 + elapsed;
        p.xrot("right_arm", -0.5);
        p.yrot("right_arm", 0.2);
        p.zrot("right_arm", roll);
        p.xrot("left_arm", -0.5);
        p.yrot("left_arm", -0.2);
        p.zrot("left_arm", -roll);
    }
}

/// NEA BurningAnimation (priority 400): the head wobbles and the free arms
/// flail.
fn apply_burning(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().burning) || st.fire_resistance {
        return;
    }
    if fire_ticks() <= 0 {
        return;
    }
    let t = st.age;
    p.get("head")[4] += t.msin() * 0.1;
    let arm_height = t.msin() * 0.1;
    nea_flail(p, st, |p| {
        p.xrot("right_arm", -2.6 + arm_height);
        p.yrot("right_arm", -0.2);
        p.zrot("right_arm", -0.3);
    }, |p| {
        p.xrot("left_arm", -2.6 - arm_height);
        p.yrot("left_arm", 0.2);
        p.zrot("left_arm", 0.3);
    });
}

/// NEA FreezingAnimation (priority 400): in powder snow without Fire
/// Resistance, the arms shake with a smooth random-ish roll per frame.
fn apply_freezing(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().freezing) || !in_powder_snow() || st.fire_resistance {
        return;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    let t = now as f64 / 1_000_000_000.0;
    let shake = |f1: f64, f2: f64| {
        ((t * f1).sin() * 0.5 + (t * f2).sin() * 0.5) as f32 * 0.05
    };
    let right_roll = -1.0 + shake(11.0, 17.0);
    let left_roll = 1.25 + shake(13.0, 19.0);
    nea_flail(p, st, |p| {
        p.xrot("right_arm", -0.5);
        p.yrot("right_arm", 0.2);
        p.zrot("right_arm", right_roll);
    }, |p| {
        p.xrot("left_arm", -0.6);
        p.yrot("left_arm", -0.2);
        p.zrot("left_arm", left_roll);
    });
}

/// NEA SleepAnimation (`freezeArmsInBed`): both arms lie still in the bed.
fn apply_sleep(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().sleep) || !st.sleep_pose {
        return;
    }
    p.xrot("right_arm", 0.0);
    p.yrot("right_arm", 0.0);
    p.zrot("right_arm", 0.0);
    p.xrot("left_arm", 0.0);
    p.yrot("left_arm", 0.0);
    p.zrot("left_arm", 0.0);
}

/// Apply `right`/`left` to the free arms; an arm busy with higher-priority
/// work keeps its pose.
fn nea_flail(
    p: &mut Poses,
    st: &NeaState,
    right: impl Fn(&mut Poses),
    left: impl Fn(&mut Poses),
) {
    let swing_right = st.swing_right_arm;
    let swinging = |left: bool| {
        st.swing > 0.0
            && match swing_right {
                Some(right) => left == !right,
                None => left == st.main_arm_left,
            }
    };
    let using_left = st.using_item > 0 && (st.using_off_hand != st.main_arm_left);
    let using_right = st.using_item > 0 && !using_left;
    let busy = |left: bool| {
        swinging(left)
            || (left && using_left)
            || (!left && using_right)
            || st.swap_at > 0.0
            || st.climbing
            || st.boat
            || st.spear
    };
    if !busy(false) {
        right(p);
    }
    if !busy(true) {
        left(p);
    }
}
