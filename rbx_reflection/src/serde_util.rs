use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Serialize, Serializer};

pub(crate) fn ordered_map<S, K, V>(value: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Hash + Ord + Serialize,
    V: Serialize,
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut ordered: Vec<_> = value.iter().collect();
    ordered.sort_unstable_by_key(|&(k, _)| k);
    let mut map = serializer.serialize_map(Some(ordered.len()))?;
    for (key, value) in ordered {
        map.serialize_entry(key, value)?;
    }
    map.end()
}

pub(crate) fn ordered_set<S, V>(value: &HashSet<V>, serializer: S) -> Result<S::Ok, S::Error>
where
    V: Hash + Ord + Serialize,
    S: Serializer,
{
    let mut ordered: Vec<_> = value.iter().collect();
    ordered.sort_unstable();
    ordered.serialize(serializer)
}
