# Not Enough Animations for Horizon

Original mod by **tr7zw**. Ported to Horizon by **romaeboj**.

A native port of [Not Enough Animations](https://github.com/tr7zw/NotEnoughAnimations)
to Horizon. It hooks the game's player-model setup through Horizon's native
mixin API, the same place the original mixes into Minecraft's player model.

**0.2.76 is listed in the Horizon mod registry, built for the latest Horizon
(`fdc87ae`), and fixes the reported bugs.**

## Ported animations

- Eat / Drink — the hand lifts to the mouth while eating or drinking.
- Ladder — climbing animation with swinging arms and legs.
- Boat — rowing animation, each oar sweeping with its own timing.
- Item Swap — the arms cross as the hands trade items.
- Burning — the head wobbles and the free arms flail while on fire.
- Freezing — the arms shake in powder snow.
- Crawling — replacing the swimming pose on land with crawling.
- No Sleep Movement — the arms lie still in bed.
- Maps — filled maps render in the hand; the back is paper.
- Bow fixes — the other hand's item hides while a bow or crossbow is drawn or
  charged. A shield never hides, and a crossbow hides a bow paired with it.

Every animation has its own on/off toggle in `Options → Mods → Not Enough
Animations`, named in the same languages as the original mod (en, de, fr, pl,
pt, ru, sv, tr, uk, zh). Two extra toggles are **In-World Map Rendering** and
**Bow Animation Fixes**.

## What's new in 0.2.76

- Updated to Horizon `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca`.
- Filled maps render on one side only, with paper around the content and on the
  back; no z-fighting.
- Maps show in the inventory player preview.
- Map pose no longer follows head pitch, and the right hand matches the left.
- Empty Map stays a plain vanilla flat item.
- Offhand items stay hidden for bows and crossbows, except shields.
- A crossbow also hides a bow in the other hand while charged or drawn.
- New toggles: In-World Map Rendering, Bow Animation Fixes.

# Install

In the Horizon launcher: open the **Mods** page, find **Not Enough Animations**,
choose **Install**, then press **Build** on the same page.

Also available from GitHub: choose **Add from GitHub**, paste

```
https://github.com/Dbdbdndbbsbs/notenoughanimations-horizon
```

and confirm the trust notice (read the source first - native mods run with your rights). The latest release's `notenoughanimations-<version>.hnmod` is installed.

## Building from source

A native bundle builds with the Horizon launcher:

```
horizon-launcher native-build . --trust-native-code
```

It pins Horizon commit `fdc87aecdbd116bf1e08adcd062fcbaea8c2e2ca` and
horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

## License

This port keeps the original's license: **tr7zw Protective License** — free to
use, modify and compile, **not** for commercial advantage or monetary
compensation. The text is in `LICENSE`; the original work is tr7zw's
NotEnoughAnimations.
