// Not Enough Animations' own half: the mod's settings, one toggle per
// animation (the names are the original mod's, translated into every
// language the original ships). No game types here; `glue.rs` applies it.
// This file is also the bundle's Cargo library, so its tests run without a
// Horizon build.

pub const ID: &str = "notenoughanimations";
pub const NAME: &str = "Not Enough Animations";
pub const VERSION: &str = "0.2.76";
pub const DESCRIPTION: &str = "Smoother first-person eating and drinking.";

const KEY_ON: &str = "enabled";
const KEY_EAT: &str = "eatDrink";
const KEY_LADDER: &str = "ladder";
const KEY_BOAT: &str = "boat";
const KEY_SWAP: &str = "itemSwap";
const KEY_BURN: &str = "burning";
const KEY_FREEZE: &str = "freezing";
const KEY_CRAWL: &str = "crawling";
const KEY_SLEEP: &str = "sleep";
const KEY_MAPS: &str = "inWorldMapRendering";
const KEY_BOW_FIXES: &str = "offhandHiding";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// Master switch: `false` turns every animation off.
    pub enabled: bool,
    pub eat_drink: bool,
    pub ladder: bool,
    pub boat: bool,
    pub item_swap: bool,
    pub burning: bool,
    pub freezing: bool,
    pub crawling: bool,
    pub sleep: bool,
    /// In-world map rendering: the held map is drawn as a 3D map.
    pub maps: bool,
    /// Bow animation fixes: the other hand's item hides while a bow or
    /// crossbow is drawn or charged.
    pub offhand_hiding: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
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

fn is_on(read: &impl Fn(&str) -> Option<String>, key: &str, def: bool) -> bool {
    read(key).map_or(def, |v| v != "false")
}

impl Settings {
    /// `false` turns the animation off; anything else (or nothing) keeps it.
    pub fn load(read: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: is_on(&read, KEY_ON, true),
            eat_drink: is_on(&read, KEY_EAT, true),
            ladder: is_on(&read, KEY_LADDER, true),
            boat: is_on(&read, KEY_BOAT, true),
            item_swap: is_on(&read, KEY_SWAP, true),
            burning: is_on(&read, KEY_BURN, true),
            freezing: is_on(&read, KEY_FREEZE, true),
            crawling: is_on(&read, KEY_CRAWL, true),
            sleep: is_on(&read, KEY_SLEEP, true),
            maps: is_on(&read, KEY_MAPS, true),
            offhand_hiding: is_on(&read, KEY_BOW_FIXES, true),
        }
    }

    pub fn saved(&self) -> Vec<(String, String)> {
        let f = |on: bool| if on { "true" } else { "false" }.into();
        vec![
            (KEY_ON.into(), f(self.enabled)),
            (KEY_EAT.into(), f(self.eat_drink)),
            (KEY_LADDER.into(), f(self.ladder)),
            (KEY_BOAT.into(), f(self.boat)),
            (KEY_SWAP.into(), f(self.item_swap)),
            (KEY_BURN.into(), f(self.burning)),
            (KEY_FREEZE.into(), f(self.freezing)),
            (KEY_CRAWL.into(), f(self.crawling)),
            (KEY_SLEEP.into(), f(self.sleep)),
            (KEY_MAPS.into(), f(self.maps)),
            (KEY_BOW_FIXES.into(), f(self.offhand_hiding)),
        ]
    }

    /// The page's buttons: one per animation (the master switch is not
    /// shown), in the same order as `click`.
    pub fn click(&mut self, button: usize) {
        let field = match button {
            0 => &mut self.eat_drink,
            1 => &mut self.ladder,
            2 => &mut self.boat,
            3 => &mut self.item_swap,
            4 => &mut self.burning,
            5 => &mut self.freezing,
            6 => &mut self.crawling,
            7 => &mut self.sleep,
            8 => &mut self.maps,
            9 => &mut self.offhand_hiding,
            _ => return,
        };
        *field = !*field;
    }

    /// The labels for `lang` (the original's `text.nea.enable.*` keys; a
    /// language the original does not ship falls back to English).
    pub fn labels(&self, lang: &str) -> Vec<String> {
        let names = NEAR_LANGS
            .iter()
            .find(|(l, _)| *l == lang)
            .map(|(_, n)| *n)
            .unwrap_or(NEAR_LANGS[3].1);
        let states = [
            self.eat_drink,
            self.ladder,
            self.boat,
            self.item_swap,
            self.burning,
            self.freezing,
            self.crawling,
            self.sleep,
            self.maps,
            self.offhand_hiding,
        ];
        names
            .iter()
            .zip(states)
            .map(|(n, on)| format!("{n}: {}", if on { "On" } else { "Off" }))
            .collect()
    }
}

/// The original mod's animation names, `[eatDrink, ladder, boat, itemSwap,
/// burning, freezing, crawling, freezeArmsInBed, maps, offhandHiding]`, from its `lang/` files
/// (missing keys fall back to English).
const NEAR_LANGS: &[(&str, [&str; 10])] = &[
    ("de_at", ["Essen/Trinken Animation", "Leiter-Animation", "Boot Rudern Animation", "Gegenstandstausch-Animation", "Brennende Animation", "Einfrier-Animation", "Kriech-Animation", "Keine Bewegung im Schlaf", "In-Welt Karten Darstellung", "Korrekturen der Bogenanimation"]),
    ("de_ch", ["Essen/Trinken Animation", "Leiter-Animation", "Boot Rudern Animation", "Gegenstandstausch-Animation", "Brennende Animation", "Einfrier-Animation", "Kriech-Animation", "Keine Bewegung im Schlaf", "In-Welt Karten Darstellung", "Korrekturen der Bogenanimation"]),
    ("de_de", ["Essen/Trinken Animation", "Leiter-Animation", "Boot Rudern Animation", "Gegenstandstausch-Animation", "Brennende Animation", "Einfrier-Animation", "Kriech-Animation", "Keine Bewegung im Schlaf", "In-Welt Karten Darstellung", "Korrekturen der Bogenanimation"]),
    ("en_us", ["Eat/Drink Animation", "Ladder Animation", "Boat Rowing Animation", "Item Swap Animation", "Burning Animation", "Freezing Animation", "Crawling Animation", "No Sleep Movement", "In-World Map Rendering", "Bow Animation Fixes"]),
    ("fr_fr", ["Animation de manger/boire", "Animation d'escalade des \u{e9}chelles", "Animation de rame de bateau", "Animation d'\u{e9}change d'objets", "Burning Animation", "Animation de cong\u{e9}lation", "Animation de ramper", "Pas de mouvement en dormant", "Rendu de la carte dans le monde", "Corrections d'animation de la seconde main"]),
    ("pl_pl", ["Animacja jedzenia/picia", "Animacja drabiny", "Animacja wios\u{142}owania \u{142}odzi\u{105}", "Animacja zamiany przedmiot\u{f3}w", "Animacja p\u{142}oni\u{119}cia", "Animacja zamarzania", "Animacja czo\u{142}gania", "Bez ruchu podczas snu", "Renderowanie mapy w \u{15b}wiecie", "Poprawki animacji \u{142}uku"]),
    ("pt_br", ["Anima\u{e7}\u{e3}o de Comer/Beber", "Anima\u{e7}\u{e3}o na Escada", "Anima\u{e7}\u{e3}o de Remar", "Anima\u{e7}\u{e3}o de Trocar Itens", "Anima\u{e7}\u{e3}o de Queimando", "Anima\u{e7}\u{e3}o de Congelamento", "Anima\u{e7}\u{e3}o de Rastejar", "Congelar Movimento Dormindo", "Renderizar Mapas no Mundo", "Corrigir Anima\u{e7}\u{e3}o com Arco"]),
    ("pt_pt", ["Anima\u{e7}\u{e3}o de Comer/Beber", "Anima\u{e7}\u{e3}o na Escada", "Anima\u{e7}\u{e3}o de Remar", "Anima\u{e7}\u{e3}o de Trocar Itens", "Burning Animation", "Anima\u{e7}\u{e3}o de Congelar", "Anima\u{e7}\u{e3}o de Ratejar", "N\u{e3}o Se Mexer a Dormir", "Mostrar Mapa a Todos", "Corrigir Anima\u{e7}\u{f5}es do Arco"]),
    ("ru_ru", ["\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{435}\u{434}\u{44b}/\u{43f}\u{438}\u{442}\u{44c}\u{44f}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{43b}\u{435}\u{441}\u{442}\u{43d}\u{438}\u{446}\u{44b}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{433}\u{440}\u{435}\u{431}\u{43b}\u{438}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{441}\u{43c}\u{435}\u{43d}\u{44b} \u{440}\u{443}\u{43a}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{433}\u{43e}\u{440}\u{435}\u{43d}\u{438}\u{44f}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{437}\u{430}\u{43c}\u{435}\u{440}\u{437}\u{430}\u{43d}\u{438}\u{44f}", "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{43f}\u{43e}\u{43b}\u{437}\u{430}\u{43d}\u{438}\u{44f}", "\u{411}\u{435}\u{437} \u{434}\u{432}\u{438}\u{436}\u{435}\u{43d}\u{438}\u{44f} \u{432} \u{43a}\u{440}\u{43e}\u{432}\u{430}\u{442}\u{438}", "\u{41e}\u{442}\u{43e}\u{431}\u{440}\u{430}\u{436}\u{435}\u{43d}\u{438}\u{435} \u{43a}\u{430}\u{440}\u{442}\u{44b} \u{432} \u{43c}\u{438}\u{440}\u{435}", "\u{418}\u{441}\u{43f}\u{440}\u{430}\u{432}\u{43b}\u{435}\u{43d}\u{438}\u{44f} \u{430}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{438} \u{43b}\u{443}\u{43a}\u{430}"]),
    ("sv_se", ["\u{c4}t/Drick Animation", "Stegeanimation", "B\u{e5}troddanimation", "F\u{f6}rem\u{e5}lsbyteanimation", "Burning Animation", "Frysanimation", "Krypanimation", "Ingen S\u{f6}mnr\u{f6}relse", "Kartrendering i v\u{e4}rlden", "B\u{e5}geanimationsfixar"]),
    ("tr_tr", ["Yeme/\u{130}\u{e7}me Animasyonu", "Merdiven Animasyonu", "Tekne K\u{fc}rek \u{c7}ekme Animasyonu", "E\u{15f}ya De\u{11f}i\u{15f}tirme Animasyonu", "Yanma Animasyonu", "Donma Animasyonu", "S\u{fc}r\u{fc}nme Animasyonu", "Uyku Hareketi Yok", "D\u{fc}nya \u{130}\u{e7}i Harita \u{130}\u{15f}leme", "Yay Animasyonu D\u{fc}zeltmeleri"]),
    ("uk_ua", ["\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{441}\u{43f}\u{43e}\u{436}\u{438}\u{432}\u{430}\u{43d}\u{43d}\u{44f}/\u{43f}\u{438}\u{442}\u{442}\u{44f}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{434}\u{440}\u{430}\u{431}\u{438}\u{43d}\u{438}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{432}\u{435}\u{441}\u{43b}\u{443}\u{432}\u{430}\u{43d}\u{43d}\u{44f} \u{43d}\u{430} \u{447}\u{43e}\u{432}\u{43d}\u{456}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{437}\u{43c}\u{456}\u{43d}\u{438} \u{43f}\u{440}\u{435}\u{434}\u{43c}\u{435}\u{442}\u{456}\u{432}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{433}\u{43e}\u{440}\u{456}\u{43d}\u{43d}\u{44f}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{43e}\u{431}\u{43c}\u{43e}\u{440}\u{43e}\u{436}\u{443}\u{432}\u{430}\u{43d}\u{43d}\u{44f}", "\u{410}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{44f} \u{43f}\u{43e}\u{432}\u{437}\u{430}\u{43d}\u{43d}\u{44f}", "\u{411}\u{435}\u{437} \u{440}\u{443}\u{445}\u{456}\u{432} \u{43f}\u{456}\u{434} \u{447}\u{430}\u{441} \u{441}\u{43d}\u{443}", "\u{412}\u{456}\u{437}\u{443}\u{430}\u{43b}\u{456}\u{437}\u{430}\u{446}\u{456}\u{44f} \u{43c}\u{430}\u{43f}\u{438} \u{441}\u{432}\u{456}\u{442}\u{443}", "\u{412}\u{438}\u{43f}\u{440}\u{430}\u{432}\u{43b}\u{435}\u{43d}\u{43d}\u{44f} \u{430}\u{43d}\u{456}\u{43c}\u{430}\u{446}\u{456}\u{457} \u{43b}\u{443}\u{43a}\u{430}"]),
    ("zh_cn", ["\u{5403}\u{559d}\u{52a8}\u{753b}", "\u{722c}\u{68af}\u{52a8}\u{753b}", "\u{5212}\u{8239}\u{52a8}\u{753b}", "\u{7269}\u{54c1}\u{4ea4}\u{6362}\u{52a8}\u{753b}", "\u{7740}\u{706b}\u{52a8}\u{753b}", "\u{51b0}\u{51bb}\u{52a8}\u{753b}", "\u{722c}\u{884c}\u{52a8}\u{753b}", "\u{7761}\u{89c9}\u{65f6}\u{624b}\u{81c2}\u{4e0d}\u{52a8}", "\u{4e16}\u{754c}\u{4e2d}\u{663e}\u{793a}\u{5730}\u{56fe}", "\u{5f13}\u{52a8}\u{753b}\u{4fee}\u{590d}"]),
    ("zh_tw", ["\u{9032}\u{98df}/\u{98f2}\u{98df}\u{52d5}\u{756b}", "\u{722c}\u{68af}\u{52d5}\u{756b}", "\u{5212}\u{8239}\u{52d5}\u{756b}", "\u{7269}\u{54c1}\u{4ea4}\u{63db}\u{52d5}\u{756b}", "Burning Animation", "\u{51b0}\u{51cd}\u{52d5}\u{756b}", "\u{6f5b}\u{884c}\u{52d5}\u{756b}", "\u{7761}\u{89ba}\u{6642}\u{4e0d}\u{79fb}\u{52d5}", "\u{4e16}\u{754c}\u{5167}\u{5730}\u{5716}\u{7e6a}\u{88fd}", "\u{5f13}\u{52d5}\u{756b}\u{4fee}\u{5fa9}"]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_by_default() {
        let s = Settings::load(|_| None);
        assert!(s.enabled);
        assert!(s.eat_drink);
        assert!(s.sleep);
        assert!(s.maps);
        assert!(s.offhand_hiding);
        assert_eq!(s.labels("en_us").len(), 10);
    }

    #[test]
    fn false_turns_each_off_and_back() {
        let mut s = Settings::load(|_| Some("false".into()));
        assert!(!s.enabled);
        assert!(!s.eat_drink);
        s.click(0);
        assert!(s.eat_drink);
        s.click(0);
        assert!(!s.eat_drink);
    }

    #[test]
    fn labels_follow_language_and_state() {
        let s = Settings::default();
        assert_eq!(s.labels("en_us")[0], "Eat/Drink Animation: On");
        assert_eq!(s.labels("ru_ru")[6], "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{43f}\u{43e}\u{43b}\u{437}\u{430}\u{43d}\u{438}\u{44f}: On");
        let mut off = Settings::default();
        off.click(6);
        assert_eq!(off.labels("ru_ru")[6], "\u{410}\u{43d}\u{438}\u{43c}\u{430}\u{446}\u{438}\u{44f} \u{43f}\u{43e}\u{43b}\u{437}\u{430}\u{43d}\u{438}\u{44f}: Off");
        // Unknown language falls back to English.
        assert_eq!(s.labels("xx_xx")[4], "Burning Animation: On");
        assert_eq!(s.labels("en_us")[8], "In-World Map Rendering: On");
        assert_eq!(s.labels("en_us")[9], "Bow Animation Fixes: On");
    }

    #[test]
    fn saved_round_trips() {
        let s = Settings {
            enabled: false,
            crawling: false,
            maps: false,
            ..Settings::default()
        };
        let saved = s.saved();
        let loaded = Settings::load(|k| saved.iter().find(|(a, _)| a == k).map(|(_, v)| v.clone()));
        assert_eq!(loaded.enabled, false);
        assert_eq!(loaded.crawling, false);
        assert_eq!(loaded.maps, false);
        assert!(loaded.burning);
    }

    #[test]
    fn every_language_has_all_ten_names() {
        for (lang, names) in NEAR_LANGS {
            assert_eq!(names.len(), 10, "{lang}");
            for n in names {
                assert!(!n.is_empty(), "{lang}");
            }
        }
    }
}

