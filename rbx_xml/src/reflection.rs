use std::collections::HashMap;

use maplit::hashmap;
use lazy_static::lazy_static;

lazy_static! {
    /// A map from 'canonical names' (properties exposed to Lua reflection) to
    /// names used in XML serialization, since these don't necessarily line up.
    ///
    /// `rbx_xml` will map between names automatically, so that consumers of
    /// `rbx_dom_weak` and related crates should be able to work with just familiar
    /// names.
    pub static ref CANONICAL_TO_XML_NAME: HashMap<&'static str, &'static str> = hashmap! {
        "FormFactor" => "formFactorRaw",
        "Size" => "size",
        "Shape" => "shape",
        "MaxPlayers" => "MaxPlayersInternal",
        "PreferredPlayers" => "PreferredPlayersInternal",
    };

    /// This is a reverse map from `CANONICAL_TO_XML_NAME`. In the future, it
    /// might not be a direct inversion, since reflected names can change over
    /// time and rbx_dom_weak needs to still handle legacy content.
    pub static ref XML_TO_CANONICAL_NAME: HashMap<&'static str, &'static str> = {
        CANONICAL_TO_XML_NAME
            .iter()
            .map(|(&key, &value)| (value, key))
            .collect()
    };
}