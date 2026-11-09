// NEA AtlasSlots helper: the slot holding `key`, without marking it recently
// shown. Needed by `map_rect` so the entity pass can look up a warmed map's
// atlas rect on `&self`.
use crate::game::atlas_slots::AtlasSlots;

impl<K: PartialEq> AtlasSlots<K> {
    pub(crate) fn slot<Q: PartialEq + ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: std::borrow::Borrow<Q>,
    {
        self.held
            .iter()
            .find(|e| e.0.borrow() == key)
            .map(|e| e.1)
    }
}
