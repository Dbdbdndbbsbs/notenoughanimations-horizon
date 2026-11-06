// NEA's animation half, inserted into `mobs/anim.rs`: the settings, and the
// ported animations applied after the vanilla setup (they read the mob's own
// state instead of the `Hum` the vanilla path built). No game types here.
use super::{
    Anim, ArmPose, Mob, Poses, State, rot_lerp_rad, villager,
};
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
}
pub fn set_nea(s: NeaSettings) {
    NEA.with(|c| c.set(s));
}
pub fn set_powder_snow(on: bool) {
    POWDER_SNOW.with(|c| c.set(on));
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

fn is_player(m: &Mob) -> bool {
    matches!(m.kind.anim, Anim::Humanoid { zombie_arms: false })
}

/// Apply every ported animation, in the original's priority order, after the
/// vanilla model pose. `setup`'s RETURN injection calls this.
pub fn apply_all(p: &mut Poses, s: &State, m: &Mob) {
    if !is_player(m) {
        return;
    }
    apply_eat_drink(p, s, m);
    apply_ladder(p, s, m);
    apply_boat(p, s, m);
    apply_crawling(p, s, m);
    apply_item_swap(p, s, m);
    apply_burning(p, s, m);
    apply_freezing(p, s, m);
    apply_sleep(p, s, m);
}

/// NEA EatDrinkAnimation (priority 2500): the using arm lifts to the mouth
/// and chews. The vanilla path put the item pose down; this replaces it.
fn apply_eat_drink(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().eat_drink) || m.extra.using_item <= 0 {
        return;
    }
    let using_main = !m.extra.using_off_hand;
    let using_off = m.extra.using_off_hand;
    let main = m.extra.main_hand.as_deref();
    let off = m.extra.off_hand.as_deref();
    let special = |item: Option<&str>, using: bool| {
        if !using {
            return true;
        }
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
    let (arm, using) = if using_main && !special(main, true) {
        ("right_arm", true)
    } else if using_off && !special(off, true) {
        ("left_arm", true)
    } else {
        return;
    };
    if !using {
        return;
    }
    let t = m.extra.using_item as f32 + s.partial;
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
fn apply_ladder(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().ladder) || !m.climbing {
        return;
    }
    let phase = m.position.y as f32 * 2.0;
    let c = (phase).mcos();
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
fn apply_boat(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().boat) || !m.extra.boat {
        return;
    }
    let row = |t: f32| -1.1 - t.msin() * 0.3;
    p.xrot("right_arm", row(m.extra.boat_right_smooth));
    p.yrot("right_arm", 0.2);
    p.zrot("right_arm", 0.3);
    p.xrot("left_arm", row(m.extra.boat_left_smooth));
    p.yrot("left_arm", -0.2);
    p.zrot("left_arm", -0.3);
}

/// NEA CrawlingAnimation (priority 350): the swimming pose on land crawls.
/// The vanilla swim pose (head down) stays; arms and legs are replaced.
fn apply_crawling(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().crawling) || m.extra.swim.in_water {
        return;
    }
    let amount = m.extra.swim.amount(s.partial);
    if amount <= 0.0 {
        return;
    }
    let speed_mul = 2.5;
    let step = (s.pos * speed_mul) % 26.0;
    let step2 = (step + 13.0) % 26.0;
    let right = m.extra.swing_right_arm;
    let m_amount = if right == Some(true) { 0.0 } else { amount };
    let n_amount = if right == Some(false) { 0.0 } else { amount };
    let lerp = |a: f32, from: f32, to: f32| from + a * (to - from);
    let rotlerp = |a: f32, from: f32, to: f32| rot_lerp_rad(a, from, to);
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
    let tc = s.pos * speed_mul;
    let x = p.get("left_leg")[3];
    p.xrot("left_leg", lerp(amount, x, leg * (tc * r + std::f32::consts::PI).mcos()));
    p.zrot("left_leg", -0.150_796_4);
    let x = p.get("right_leg")[3];
    p.xrot("right_leg", lerp(amount, x, leg * (tc * r).mcos()));
    p.zrot("right_leg", 0.150_796_4);
}

/// NEA ItemSwapAnimation (priority 3500): the arms cross as the hands trade
/// their items and unroll over 10 ticks.
fn apply_item_swap(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().item_swap) || m.extra.swap_at <= 0.0 {
        return;
    }
    let elapsed = (s.age - m.extra.swap_at) / 10.0;
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
/// flail. Arms held by higher-priority work keep their pose.
fn apply_burning(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().burning) || m.fire_ticks <= 0 || m.has_effect("minecraft:fire_resistance") {
        return;
    }
    let t = s.age;
    p.get("head")[4] += t.msin() * 0.1;
    let arm_height = t.msin() * 0.1;
    nea_flail(p, s, m, |p| {
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
fn apply_freezing(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().freezing) || !in_powder_snow() {
        return;
    }
    if m.has_effect("minecraft:fire_resistance") {
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
    nea_flail(p, s, m, |p| {
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
fn apply_sleep(p: &mut Poses, s: &State, m: &Mob) {
    if !nea_on(nea().sleep) || villager::sleep_pose(m).is_none() {
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
/// work (a swing, held use, a recent swap, climbing, a boat, aiming a
/// two-handed weapon or a spear) keeps its pose.
fn nea_flail(
    p: &mut Poses,
    s: &State,
    m: &Mob,
    right: impl Fn(&mut Poses),
    left: impl Fn(&mut Poses),
) {
    let swing_right = m.extra.swing_right_arm;
    let swinging = |left: bool| {
        s.swing > 0.0
            && match swing_right {
                Some(right) => left == !right,
                None => left == m.main_arm_left(),
            }
    };
    let using_left = m.extra.using_item > 0 && (m.extra.using_off_hand != m.main_arm_left());
    let using_right = m.extra.using_item > 0 && !using_left;
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
    let busy = |left: bool| {
        swinging(left)
            || (left && using_left)
            || (!left && using_right)
            || m.extra.swap_at > 0.0
            || m.climbing
            || m.extra.boat
            || spear
    };
    if !busy(false) {
        right(p);
    }
    if !busy(true) {
        left(p);
    }
}
