// NEA's animation half, inserted into `game/client_ambient.rs`: the settings
// and the ported animations. `setup`'s HEAD hook collects the player's state
// (`NeaState`); the model-visibility HEAD hook (right after the vanilla
// `humanoid` pose) applies the animations on top.
use crate::mobs::anim::{Anim, Poses, State};
use crate::mobs::Mob;
use horizon_core::geometry::trig::Mth;
use std::cell::{Cell, RefCell};
use std::sync::Arc;

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
    pub maps: bool,
    pub offhand_hiding: bool,
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
            maps: true,
            offhand_hiding: true,
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
            maps: true,
            offhand_hiding: true,
        })
    };
    /// NEA FreezingAnimation: the local player's box touches powder snow
    /// (computed by the frame hook).
    static POWDER_SNOW: Cell<bool> = const { Cell::new(false) };
    /// NEA BurningAnimation: the local player's remaining fire ticks (the
    /// model's own `fire_ticks` is only synced for remote players).
    static FIRE_TICKS: Cell<i32> = const { Cell::new(0) };
    /// NEA LadderAnimation: the local player is on a climbable block that is
    /// a ladder or trapdoor (computed by the frame hook).
    static CLIMBING: Cell<bool> = const { Cell::new(false) };
    /// NEA LadderAnimation: the body yaw (radians) that faces the ladder,
    /// read from the ladder block's `facing` by the frame hook.
    static LADDER_FACE: Cell<f32> = const { Cell::new(0.0) };
    /// NEA LadderAnimation: the climbable block has a `facing` (ladder or
    /// trapdoor), so the body is locked and the head is clamped to it.
    static LADDER_LOCK: Cell<bool> = const { Cell::new(false) };
    /// NEA BurningAnimation: the local player is on fire without fire
    /// resistance (computed by the frame hook).
    static BURNING: Cell<bool> = const { Cell::new(false) };
    /// NEA FreezingAnimation: the local player is freezing in powder snow
    /// (computed by the frame hook).
    static FREEZING: Cell<bool> = const { Cell::new(false) };
    /// The current player's animation state, captured by the `setup` HEAD hook.
    static STATE: Cell<NeaState> = const { Cell::new(NeaState::NONE) };
    /// NEA ItemSwapAnimation: the model age when the hands last traded items.
    static SWAP_AT: Cell<f32> = const { Cell::new(0.0) };
    /// NEA BoatAnimation: riding a boat, and each paddle's state and time.
    static BOAT: Cell<(bool, [bool; 2], [f32; 2])> =
        const { Cell::new((false, [false; 2], [0.0; 2])) };
    /// The hands the frame hook saw last, to spot a trade.
    static HANDS: RefCell<(Option<Arc<str>>, Option<Arc<str>>)> =
        const { RefCell::new((None, None)) };
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
pub fn set_climbing(on: bool) {
    CLIMBING.with(|c| c.set(on));
}
pub fn set_ladder_face(yaw: f32) {
    LADDER_FACE.with(|c| c.set(yaw));
}
pub fn set_ladder_lock(on: bool) {
    LADDER_LOCK.with(|c| c.set(on));
}
pub fn set_burning(on: bool) {
    BURNING.with(|c| c.set(on));
}
pub fn set_freezing(on: bool) {
    FREEZING.with(|c| c.set(on));
}
/// Forget the local player's frame state (left the world).
pub fn reset_animation_state() {
    SWAP_AT.with(|c| c.set(0.0));
    BOAT.with(|c| c.set((false, [false; 2], [0.0; 2])));
    HANDS.with(|h| *h.borrow_mut() = (None, None));
}
/// NEA BoatAnimation: the boat the local player rides, and each paddle's
/// state and time (`AbstractBoat.rowingTimeLeft/Right`).
pub fn update_boat(boat: Option<([bool; 2], [f32; 2])>) {
    BOAT.with(|c| c.set(match boat {
        Some((paddling, paddles)) => (true, paddling, paddles),
        None => (false, [false; 2], [0.0; 2]),
    }));
}
/// `AbstractBoat.getRowingTime`: 0 when the oar is not paddling, else the
/// paddle time interpolated over the render frame.
fn rowing_time(paddling: bool, paddle: f32, partial: f32) -> f32 {
    if paddling {
        paddle - std::f32::consts::FRAC_PI_8 + partial * std::f32::consts::FRAC_PI_8
    } else {
        0.0
    }
}
/// NEA ItemSwapAnimation: notice the hands trading their items, and remember
/// the age it happened at.
pub fn update_hands(age: f32, main: Option<Arc<str>>, off: Option<Arc<str>>) {
    HANDS.with(|h| {
        let (last_main, last_off) = h.borrow().clone();
        if last_main != last_off
            && main == last_off
            && off == last_main
            && (main.is_some() || off.is_some())
        {
            SWAP_AT.with(|c| c.set(age));
        }
        *h.borrow_mut() = (main, off);
    });
}
fn swap_at() -> f32 {
    SWAP_AT.with(|c| c.get())
}
fn boat() -> (bool, [bool; 2], [f32; 2]) {
    BOAT.with(|c| c.get())
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
    pub is_local: bool,
    pub partial: f32,
    pub pos: f32,
    pub y: f32,
    pub age: f32,
    pub pitch: f32,
    pub swing: f32,
    pub using_item: i32,
    pub using_off_hand: bool,
    pub swap_at: f32,
    pub climbing: bool,
    pub ladder_face: f32,
    pub ladder_lock: bool,
    pub head_yaw: f32,
    pub boat: bool,
    pub boat_left: f32,
    pub boat_right: f32,
    pub swim: f32,
    pub swim_in_water: bool,
    pub main_arm_left: bool,
    pub swing_right_arm: Option<bool>,
    pub main_special: bool,
    pub off_special: bool,
    pub main_map: bool,
    pub off_map: bool,
    pub off_empty: bool,
    pub spear: bool,
    pub sleep_pose: bool,
    pub fire_resistance: bool,
    pub burning: bool,
    pub freezing: bool,
}
impl NeaState {
    pub const NONE: Self = Self {
        is_player: false,
        is_local: false,
        partial: 0.0,
        pos: 0.0,
        y: 0.0,
        age: 0.0,
        pitch: 0.0,
        swing: 0.0,
        using_item: 0,
        using_off_hand: false,
        swap_at: 0.0,
        climbing: false,
        ladder_face: 0.0,
        ladder_lock: false,
        head_yaw: 0.0,
        boat: false,
        boat_left: 0.0,
        boat_right: 0.0,
        swim: 0.0,
        swim_in_water: false,
        main_arm_left: false,
        swing_right_arm: None,
        main_special: false,
        off_special: false,
        main_map: false,
        off_map: false,
        off_empty: true,
        spear: false,
        sleep_pose: false,
        fire_resistance: false,
        burning: false,
        freezing: false,
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
        // Only a filled map gets the map-holding pose. An empty map stays a
        // plain item and is held like one.
        let is_map =
            |item: Option<&str>| matches!(item, Some("minecraft:filled_map"));
        let (boat, paddling, paddles) = boat();
        let is_player = matches!(m.kind.anim, Anim::Humanoid { zombie_arms: false });
        let is_local = m.id == 0;
        // The local player's climbing/freezing/burning state comes from the
        // frame hook, which reads PlayerMotion (synced fields are unavailable
        // for the local model). Remote players and other mobs use their own
        // synced fields.
        let climbing = if is_local && is_player {
            CLIMBING.with(|c| c.get())
        } else {
            m.climbing
        };
        let burning = if is_local && is_player {
            BURNING.with(|c| c.get())
        } else {
            m.is_on_fire() && !m.has_effect("minecraft:fire_resistance")
        };
        let freezing = if is_local && is_player {
            FREEZING.with(|c| c.get())
        } else {
            false
        };
        Self {
            is_player,
            is_local,
            partial: s.partial,
            pos: s.pos,
            y: m.position.y as f32,
            age: s.age,
            pitch: s.x_rot,
            swing: s.swing,
            using_item: m.extra.using_item,
            using_off_hand: m.extra.using_off_hand,
            swap_at: swap_at(),
            climbing,
            ladder_face: if climbing {
                LADDER_FACE.with(|c| c.get())
            } else {
                0.0
            },
            // Remote players have no local ladder block; keep their vanilla
            // body and head instead of using the local player's lock.
            ladder_lock: climbing && is_local && is_player && LADDER_LOCK.with(|c| c.get()),
            // `State.y_rot` is the head relative to the (locked) body, so it
            // is already the sideways look that the ladder animation clamps.
            head_yaw: s.y_rot.to_radians(),
            boat,
            boat_left: rowing_time(paddling[0], paddles[0], s.partial),
            boat_right: rowing_time(paddling[1], paddles[1], s.partial),
            swim: m.extra.swim.amount(s.partial),
            swim_in_water: m.extra.swim.in_water,
            main_arm_left: m.main_arm_left(),
            swing_right_arm: m.extra.swing_right_arm,
            main_special: special(m.extra.main_hand.as_deref()),
            off_special: special(m.extra.off_hand.as_deref()),
            main_map: is_map(m.extra.main_hand.as_deref()),
            off_map: is_map(m.extra.off_hand.as_deref()),
            off_empty: m.extra.off_hand.is_none(),
            spear,
            sleep_pose: crate::mobs::villager::sleep_pose(m).is_some(),
            fire_resistance: m.has_effect("minecraft:fire_resistance"),
            burning,
            freezing,
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
    // Lowest priority first: a later animation overwrites the arms it owns,
    // as the original's priority order does (Map 300, Crawling 350,
    // Burning/Freezing 400, Ladder 1000, Boat 1500, Eat/Drink 2500,
    // Item Swap 3500).
    apply_map_holding(p, &st);
    apply_crawling(p, &st);
    apply_burning(p, &st);
    apply_freezing(p, &st);
    apply_ladder(p, &st);
    apply_boat(p, &st);
    apply_eat_drink(p, &st);
    apply_item_swap(p, &st);
    apply_sleep(p, &st);
}

/// NEA MapHoldingAnimation (priority 300): the arms hold the map in front,
/// both hands when the off hand is empty, otherwise only the hand with the
/// map. The original's pitch-driven formulas.
fn apply_map_holding(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().maps) {
        return;
    }
    let both = st.main_map && st.off_empty;
    let both_maps = st.main_map && st.off_map;
    let main_only = st.main_map && !st.off_empty && !st.off_map;
    let off_only = !st.main_map && st.off_map;
    if !(both || both_maps || main_only || off_only) {
        return;
    }
    // `EntityUtil.getXRot` is the head pitch in degrees.
    let t = (90.0 - st.pitch) / 180.0;
    let lerp = |a: f32, from: f32, to: f32| from + a * (to - from);
    // The original's pitch-driven formulas swing the arms (and the maps in
    // them) as the head tilts, so two maps converge and diverge when looking
    // up and down. Pin the pose to level gaze (t = 0.5): the map stays fixed
    // in the hands no matter the head angle.
    let (x, y) = if both {
        (-lerp(0.5, 0.7, 0.9), lerp(0.5, -0.3, -0.2))
    } else {
        (-lerp(0.5, 0.5, 1.5), 0.0)
    };
    let arm = |p: &mut Poses, right: bool| {
        let name = if right { "right_arm" } else { "left_arm" };
        p.xrot(name, x);
        p.yrot(name, if right { y } else { -y });
        p.zrot(name, if right { 0.3 } else { -0.3 });
    };
    if both {
        arm(p, true);
        arm(p, false);
    } else if both_maps {
        // A map in each hand: the original only poses the main arm, leaving
        // the off-hand map floating. Pose both arms with the one-handed
        // formula so each hand holds its own map.
        arm(p, true);
        arm(p, false);
    } else if main_only {
        arm(p, !st.main_arm_left);
    } else {
        // The off arm is the opposite of the main arm.
        arm(p, st.main_arm_left);
    }
}

/// NEA EatDrinkAnimation (priority 2500): the using arm lifts to the mouth
/// and chews. The classic NEA port formula.
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

/// NEA LadderAnimation (priority 1000): the original tr7zw animation. Arms
/// hang at -1.7, legs bend at -1.0 so the feet rest on the rungs; all parts
/// swing with `cos(worldY * 2) * 0.35`, left limbs mirror yaw/z. The frame
/// hook locks the body to the ladder's `facing`; the head turns with the
/// camera, clamped to ±90° of the body.
fn apply_ladder(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().ladder) || !st.climbing {
        return;
    }
    let rotation = -(st.y * 2.0).cos() * 0.35;
    // Right arm.
    p.xrot("right_arm", -1.7 - rotation);
    p.yrot("right_arm", -0.2);
    p.zrot("right_arm", 0.3);
    // Left arm (mirrored yaw/z).
    p.xrot("left_arm", -1.7 + rotation);
    p.yrot("left_arm", 0.2);
    p.zrot("left_arm", -0.3);
    // Right leg: bent, feet resting on the rungs.
    p.xrot("right_leg", -1.0 - rotation);
    p.yrot("right_leg", -0.2);
    p.zrot("right_leg", 0.3);
    // Left leg (mirrored yaw/z).
    p.xrot("left_leg", -1.0 + rotation);
    p.yrot("left_leg", 0.2);
    p.zrot("left_leg", -0.3);
    // The body's world yaw is locked to the ladder by the frame hook. The
    // head follows the camera relative to that body, clamped to +-90 degrees
    // so a backwards climb turns the head sideways instead of spinning it.
    if st.ladder_lock {
        p.yrot(
            "head",
            st.head_yaw
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2),
        );
    }
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
    let elapsed = st.age - st.swap_at;
    if !(0.0..=10.0).contains(&elapsed) {
        return;
    }
    let roll = -1.0 + elapsed / 10.0;
    p.xrot("right_arm", -0.5);
    p.yrot("right_arm", 0.2);
    p.zrot("right_arm", roll);
    p.xrot("left_arm", -0.5);
    p.yrot("left_arm", -0.2);
    p.zrot("left_arm", -roll);
}

/// NEA BurningAnimation (priority 400): the head wobbles and the free arms
/// flail.
fn apply_burning(p: &mut Poses, st: &NeaState) {
    if !nea_on(nea().burning) || !st.burning {
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
    if !nea_on(nea().freezing) || !st.freezing {
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
/// work keeps its pose. The swap flag is only busy while the 10-tick item
/// swap timer is still running; once it expires the arm is free again.
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
    let swap_busy = st.swap_at > 0.0 && st.age - st.swap_at < 10.0;
    let busy = |left: bool| {
        swinging(left)
            || (left && using_left)
            || (!left && using_right)
            || swap_busy
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
