use std::collections::BTreeSet;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollisionGroupError {
    #[error(transparent)]
    ParseIntError {
        #[from]
        source: std::num::ParseIntError,
    },

    #[error("failed to parse CollisionGroups")]
    ParseError,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CollisionGroups {
    groups: BTreeSet<CollisionGroup>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CollisionGroup {
    id: i32,
    mask: i32,
    name: String,
}

impl PartialOrd for CollisionGroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CollisionGroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl CollisionGroups {
    pub fn decode(encoded: &str) -> Result<CollisionGroups, CollisionGroupError> {
        use CollisionGroupError::ParseError;

        let groups = encoded
            .split('\\')
            .into_iter()
            .map(|encoded_group| {
                let mut raw_group = encoded_group.split('^');

                Ok(CollisionGroup {
                    name: raw_group.next().ok_or(ParseError)?.to_string(),
                    id: raw_group.next().ok_or(ParseError)?.parse::<i32>()?,
                    mask: raw_group.next().ok_or(ParseError)?.parse::<i32>()?,
                })
            })
            .collect::<Result<BTreeSet<_>, CollisionGroupError>>()?;

        Ok(CollisionGroups { groups })
    }

    pub fn encode(&self) -> String {
        self.groups
            .iter()
            .map(|collision_group| {
                format!(
                    "{}^{}^{}",
                    collision_group.name, collision_group.id, collision_group.mask
                )
            })
            .collect::<Vec<_>>()
            .join("\\")
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;

    use serde::{
        de::{self, SeqAccess},
        ser::SerializeSeq,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SerializedCollisionGroup<'a> {
        name: &'a str,
        id: i32,

        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        not_collidable_with: Vec<&'a str>,
    }

    impl Serialize for CollisionGroups {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_seq(Some(self.groups.len()))?;

            for group in self.groups.iter() {
                let not_collidable_with = (0..31)
                    .filter(|id| ((group.mask & (1 << id)) >> id == 0))
                    .filter_map(|id| {
                        let group_from_id = self.groups.iter().find(|group| group.id == id);

                        match group_from_id {
                            Some(group) => Some(group.name.as_str()),
                            None if id == 0 => Some("Default"),
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>();

                state.serialize_element(&SerializedCollisionGroup {
                    name: group.name.as_str(),
                    id: group.id,
                    not_collidable_with,
                })?;
            }

            state.end()
        }
    }

    struct CollisionGroupsVisitor;

    impl<'de> de::Visitor<'de> for CollisionGroupsVisitor {
        type Value = CollisionGroups;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a list of CollisionGroups")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut serialized_groups: Vec<SerializedCollisionGroup> = Vec::new();

            while let Some(group) = seq.next_element()? {
                serialized_groups.push(group);
            }

            if serialized_groups
                .iter()
                .find(|group| group.name == "Default")
                .is_none()
            {
                serialized_groups.insert(
                    0,
                    SerializedCollisionGroup {
                        name: "Default",
                        id: 0,
                        not_collidable_with: serialized_groups
                            .iter()
                            .filter_map(|group| {
                                group
                                    .not_collidable_with
                                    .iter()
                                    .find(|name| *name == &"Default")
                            })
                            .copied()
                            .collect(),
                    },
                );
            };

            let groups = serialized_groups
                .iter()
                .map(|serialized_group| {
                    let mask =
                        serialized_group
                            .not_collidable_with
                            .iter()
                            .fold(-1, |mask, group_name| {
                                let found_group = serialized_groups
                                    .iter()
                                    .find(|group| group.name == *group_name);

                                match found_group {
                                    Some(group) => mask & !(1 << group.id),
                                    None => mask,
                                }
                            });

                    CollisionGroup {
                        name: serialized_group.name.to_string(),
                        id: serialized_group.id,
                        mask,
                    }
                })
                .collect::<BTreeSet<CollisionGroup>>();

            Ok(CollisionGroups { groups })
        }
    }

    impl<'de> Deserialize<'de> for CollisionGroups {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(CollisionGroupsVisitor)
        }
    }
}

#[cfg(test)]
fn create_test_groups() -> CollisionGroups {
    let mut groups = BTreeSet::new();

    groups.insert(CollisionGroup {
        name: "A".to_string(),
        id: 2,
        mask: -262,
    });

    groups.insert(CollisionGroup {
        name: "B".to_string(),
        id: 5,
        mask: -321,
    });

    groups.insert(CollisionGroup {
        name: "C".to_string(),
        id: 6,
        mask: -33,
    });

    groups.insert(CollisionGroup {
        name: "D".to_string(),
        id: 8,
        mask: -37,
    });

    CollisionGroups { groups }
}

#[cfg(all(test, feature = "serde"))]
mod serde_test {
    use super::*;

    #[test]
    fn serialize_json() {
        let serialized = serde_json::to_string(&create_test_groups()).unwrap();

        assert_eq!(
            serialized,
            "[\
              {\"name\":\"A\",\"id\":2,\"notCollidableWith\":[\"Default\",\"A\",\"D\"]},\
              {\"name\":\"B\",\"id\":5,\"notCollidableWith\":[\"C\",\"D\"]},\
              {\"name\":\"C\",\"id\":6,\"notCollidableWith\":[\"B\"]},\
              {\"name\":\"D\",\"id\":8,\"notCollidableWith\":[\"A\",\"B\"]}\
            ]"
        )
    }

    #[test]
    fn round_trip_json() {
        let mut test_groups = create_test_groups();

        test_groups.groups.insert(CollisionGroup {
            name: "Default".to_string(),
            id: 0,
            mask: -2,
        });

        let serialized = serde_json::to_string(&test_groups).unwrap();
        let deserialized: CollisionGroups = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, test_groups);
    }

    #[test]
    fn empty_non_collidable_json() {
        let mut groups = BTreeSet::new();

        groups.insert(CollisionGroup {
            name: "Default".to_string(),
            id: 0,
            mask: -1,
        });

        let collision_groups = CollisionGroups { groups };
        let serialized = serde_json::to_string(&collision_groups).unwrap();
        let round_tripped: CollisionGroups = serde_json::from_str(&serialized).unwrap();

        assert_eq!(serialized, "[{\"name\":\"Default\",\"id\":0}]");
        assert_eq!(round_tripped, collision_groups);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decode() {
        let decoded = CollisionGroups::decode("A^2^-262\\B^5^-321\\C^6^-33\\D^8^-37").unwrap();
        assert_eq!(decoded, create_test_groups());
    }

    #[test]
    fn encode() {
        assert_eq!(
            "A^2^-262\\B^5^-321\\C^6^-33\\D^8^-37",
            create_test_groups().encode()
        );
    }
}
