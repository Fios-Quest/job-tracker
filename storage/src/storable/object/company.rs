use crate::storable::*;
use crate::Timestamp;
use partially::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Partial)]
#[partially(derive(Deserialize, Default))]
pub struct Company {
    #[partially(omit)]
    pub id: Uuid,
    pub name: String,
    pub date_deleted: Option<Timestamp>,
}

impl Company {
    pub fn new<S: Into<String>>(name: S) -> Company {
        Company {
            id: Uuid::new_v4(),
            name: name.into(),
            date_deleted: None,
        }
    }

    pub fn create_role<S: Into<String>>(&self, name: S, date_created: Timestamp) -> Role {
        Role::new(self, name, date_created)
    }

    pub fn create_green_flag<S: Into<String>>(&self, name: S) -> Flag {
        Flag::new_green(self, name)
    }

    pub fn create_red_flag<S: Into<String>>(&self, name: S) -> Flag {
        Flag::new_red(self, name)
    }

    pub fn create_value<N: Into<String>, D: Into<String>>(&self, name: N, description: D) -> Value {
        Value::new(self, name, description)
    }
}

impl_has_id!(Company);
impl_has_name!(Company);
impl_has_deleted!(Company);

#[cfg(test)]
mod test_helper {
    use crate::storable::Company;
    use crate::test_helper::TestHelper;

    impl TestHelper for Company {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Company::new("Company"))
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

    test_has_id!(Company);
    test_has_name!(Company);
    test_has_deleted!(Company);

    #[test]
    fn test_create_role() {
        let company = Company::new("company");
        let role = company.create_role("role", Timestamp::now());
        assert_eq!(company.get_id(), role.get_company_id());
    }

    #[test]
    fn test_create_green_flag() {
        let company = Company::new("company");
        let flag = company.create_green_flag("green flag");
        assert_eq!(company.get_id(), flag.get_company_id());
    }

    #[test]
    fn test_create_red_flag() {
        let company = Company::new("company");
        let flag = company.create_red_flag("red flag");
        assert_eq!(company.get_id(), flag.get_company_id());
    }

    #[test]
    fn test_create_value() {
        let company = Company::new("company");
        let value = company.create_value("name", "description");
        assert_eq!(value.get_company_id(), company.get_id());
        assert_eq!(value.name, "name");
        assert_eq!(value.description, "description");
    }

    #[test]
    fn test_modify_with_hashmap() {
        let mut company = Company::new("Original name");
        let original_id = company.id;

        let mut hash_map: HashMap<String, serde_json::Value> = HashMap::new();
        hash_map.insert("id".to_string(), Uuid::new_v4().to_string().into()); // This should never change
        hash_map.insert("name".to_string(), "New name".into());
        hash_map.insert("date_deleted".to_string(), "2025-07-28T00:00".into());

        let partial_company = PartialCompany::deserialize(hash_map.into_deserializer()).unwrap();
        company.apply(partial_company);

        assert_eq!(company.id, original_id);
        assert_eq!(company.name, "New name".to_string());
        assert_eq!(
            company.date_deleted,
            Some(Timestamp::from_string("2025-07-28T00:00"))
        );
    }
}
