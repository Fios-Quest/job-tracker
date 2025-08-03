use crate::storable::*;
use crate::Timestamp;
use partially::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Partial)]
#[partially(derive(Deserialize, Default), attribute(serde(default)))]
pub struct Interview {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub role_id: Uuid,
    pub name: String,
    pub notes: String,
    pub host: String,
    pub date_time: Option<Timestamp>,
    pub date_deleted: Option<Timestamp>,
}

impl Interview {
    pub fn new<R: HasId, S: Into<String>>(role: R, name: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            role_id: role.get_id(),
            name: name.into(),
            notes: String::new(),
            host: String::new(),
            date_time: None,
            date_deleted: None,
        }
    }
}

impl_has_id!(Interview);
impl_has_name!(Interview);
impl_has_role!(Interview);
impl_has_deleted!(Interview);

impl_is_partial_complete_optional_name_only!(PartialInterview);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Interview {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Interview::new(Uuid::new_v4(), "Interview"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::TestHelper;
    use paste::paste;
    use serde::de::IntoDeserializer;
    use std::collections::HashMap;

    test_has_id!(Interview);
    test_has_name!(Interview);
    test_has_role!(Interview);
    test_has_deleted!(Interview);

    #[test]
    fn test_modify_with_hashmap() {
        let mut interview = Interview::new(Uuid::new_v4(), "Interview name");
        let original_id = interview.id;
        let original_role = interview.role_id;

        let mut hash_map: HashMap<String, serde_json::Value> = HashMap::new();
        hash_map.insert("id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("role_id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("name".to_string(), "New name".into());
        hash_map.insert("notes".to_string(), "New notes".into());
        hash_map.insert("host".to_string(), "Host 1, Host 2".into());
        hash_map.insert("date_time".to_string(), "2025-07-28T00:00".into());
        hash_map.insert("date_deleted".to_string(), "2026-07-28T00:00".into());

        let partial_interview =
            PartialInterview::deserialize(hash_map.into_deserializer()).unwrap();
        interview.apply(partial_interview);

        assert_eq!(interview.id, original_id);
        assert_eq!(interview.role_id, original_role);
        assert_eq!(interview.name, "New name".to_string());
        assert_eq!(interview.notes, "New notes".to_string());
        assert_eq!(interview.host, "Host 1, Host 2".to_string());
        assert_eq!(
            interview.date_time,
            Some(Timestamp::from_string("2025-07-28T00:00"))
        );
        assert_eq!(
            interview.date_deleted,
            Some(Timestamp::from_string("2026-07-28T00:00"))
        );
    }

    #[test]
    fn test_partial_interview_is_complete_complete_interview() {
        let interview = PartialInterview {
            name: Some("Test interview".to_string()),
            notes: None,
            host: None,
            date_time: None,
            date_deleted: None,
        };
        assert!(interview.is_complete())
    }

    #[test]
    fn test_partial_interview_is_complete_missing_name() {
        let interview = PartialInterview {
            name: None,
            notes: None,
            host: None,
            date_time: None,
            date_deleted: None,
        };
        assert!(!interview.is_complete())
    }

    #[test]
    fn test_partial_interview_is_complete_empty_name() {
        let interview = PartialInterview {
            name: Some(String::new()),
            notes: None,
            host: None,
            date_time: None,
            date_deleted: None,
        };
        assert!(!interview.is_complete())
    }
}
