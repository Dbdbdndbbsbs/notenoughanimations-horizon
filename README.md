# Not Enough Animations for Horizon

Original mod by **tr7zw**. Ported to Horizon by **romaeboj**.

A native port of [Not Enough Animations](https://github.com/tr7zw/NotEnoughAnimations)
to Horizon. It patches the game's own Rust source (`crates/horizon-client`
`mobs/anim.rs`, `mobs.rs` and `game/perspective.rs`) the same way the original
mixes into Minecraft's player model.

## Ported animations

- Eat / Drink — the hand lifts to the mouth while eating or drinking.
- Ladder — climbing animation with swinging arms and legs.
- Boat — rowing animation, each oar sweeping with its own timing.
- Item Swap — the arms cross as the hands trade items.
- Burning — the head wobbles and the free arms flail while on fire.
- Freezing — the arms shake in powder snow.
- Crawling — replacing the swimming pose on land with crawling.
- No Sleep Movement — the arms lie still in bed.

Every animation has its own on/off toggle in `Options → Mods → Not Enough
Animations`, named in the same languages as the original mod (en, de, fr, pl,
pt, ru, sv, tr, uk, zh).

The latest release's `notenoughanimations-<version>.hnmod` is installed.

## Building from source

A native bundle builds with the Horizon launcher:

```
horizon-launcher native-build . --trust-native-code
```

It pins Horizon commit `66f3981f522b24c250b21523a70080965b940424` and
horizon-mixins `5a64af938ec86b00d53d16a408aabc0038fb3e57`.

## License

This port keeps the original's license: **tr7zw Protective License** — free to
use, modify and compile, **not** for commercial advantage or monetary
compensation. The text is in `LICENSE`; the original work is tr7zw's
NotEnoughAnimations.
