use crate::storable::*;
use crate::Timestamp;
use partially::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Partial)]
#[partially(derive(Deserialize, Default))]
pub struct Value {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub company_id: Uuid,
    pub name: String,
    pub description: String,
    pub date_deleted: Option<Timestamp>,
}

impl Value {
    pub fn new<C: HasId, N: Into<String>, D: Into<String>>(
        company: C,
        name: N,
        description: D,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            name: name.into(),
            description: description.into(),
            date_deleted: None,
        }
    }

    pub fn new_from_partial<C: HasId>(
        company: C,
        partial: PartialValue,
    ) -> Result<Value, IncompletePartialErrors> {
        partial.check_complete()?;

        Ok(Value {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            name: partial
                .name
                .ok_or_else(|| IncompletePartialErrors::field_error("name"))?,
            description: partial.description.unwrap_or_default(),
            date_deleted: partial.date_deleted.unwrap_or_default(),
        })
    }
}

impl_has_id!(Value);
impl_has_name!(Value);
impl_has_company!(Value);
impl_has_deleted!(Value);

impl_is_partial_complete_optional_name_only!(PartialValue);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Value {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Value::new(Uuid::new_v4(), "Value", "Description"))
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

    test_has_id!(Value);
    test_has_name!(Value);
    test_has_company!(Value);
    test_has_deleted!(Value);

    #[test]
    fn test_modify_with_hashmap() {
        let mut value = Value::new(Uuid::new_v4(), "Value name", "Value description");
        let original_id = value.id;
        let original_company = value.company_id;

        let mut hash_map: HashMap<String, serde_json::Value> = HashMap::new();
        hash_map.insert("id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("company_id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("name".to_string(), "New name".into());
        hash_map.insert("description".to_string(), "New description".into());
        hash_map.insert("date_deleted".to_string(), "2025-07-28T00:00".into());

        let partial_value = PartialValue::deserialize(hash_map.into_deserializer()).unwrap();
        value.apply(partial_value);

        assert_eq!(value.id, original_id);
        assert_eq!(value.company_id, original_company);
        assert_eq!(value.name, "New name".to_string());
        assert_eq!(value.description, "New description".to_string());
        assert_eq!(
            value.date_deleted,
            Some(Timestamp::from_string("2025-07-28T00:00"))
        );
    }

    #[test]
    fn test_partial_value_is_complete_complete_value() {
        let complete_value = PartialValue {
            name: Some("Test Value".to_string()),
            description: None,
            date_deleted: None,
        };
        assert!(complete_value.check_complete().is_ok());
    }

    #[test]
    fn test_partial_value_is_complete_missing_name() {
        let missing_name = PartialValue {
            name: None,
            description: None,
            date_deleted: None,
        };
        let error = missing_name.check_complete().unwrap_err();
        let errors = error.get_errors();
        assert!(errors.contains(&String::from("`name` is missing")));
    }

    #[test]
    fn test_partial_value_is_complete_empty_name() {
        let missing_name = PartialValue {
            name: Some(String::new()),
            description: None,
            date_deleted: None,
        };
        let error = missing_name.check_complete().unwrap_err();
        let errors = error.get_errors();
        assert!(errors.contains(&String::from("`name` is empty")));
    }
}
