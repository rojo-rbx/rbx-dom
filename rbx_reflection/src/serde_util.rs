use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use serde::{Serialize, Serializer};

pub(crate) fn ordered_map<S, K, V>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Hash + Ord + Serialize,
    V: Serialize,
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}
