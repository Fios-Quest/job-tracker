use crate::prelude::Interview;
use crate::storable::*;
use crate::Timestamp;
use partially::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Partial)]
#[partially(derive(Deserialize, Default))]
pub struct Role {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub company_id: Uuid,
    pub name: String,
    pub description: String,
    pub date_applied: Timestamp,
    pub date_deleted: Option<Timestamp>,
}

impl Role {
    pub fn new<C: HasId, S: Into<String>>(company: C, name: S, date_applied: Timestamp) -> Role {
        Role {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            name: name.into(),
            description: "".to_string(),
            date_applied,
            date_deleted: None,
        }
    }

    pub fn create_question<N: Into<String>, A: Into<String>>(
        &self,
        name: N,
        answer: A,
    ) -> Question {
        Question::new(self, name, answer)
    }

    pub fn create_interview<S: Into<String>>(&self, name: S) -> Interview {
        Interview::new(self, name)
    }
}

impl_has_id!(Role);
impl_has_name!(Role);
impl_has_company!(Role);
impl_has_deleted!(Role);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Role {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Role::new(Uuid::new_v4(), "Role", Timestamp::now()))
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

    test_has_id!(Role);
    test_has_name!(Role);
    test_has_company!(Role);
    test_has_deleted!(Role);

    #[test]
    fn test_create_question() {
        let role = Role::new(Uuid::new_v4(), "role", Timestamp::now());
        let question = role.create_question("question", "answer");
        assert_eq!(question.role_id, role.id);
        assert_eq!(question.name, "question");
        assert_eq!(question.answer, "answer");
    }

    #[test]
    fn test_create_interview() {
        let role = Role::new(Uuid::new_v4(), "role", Timestamp::now());
        let interview = role.create_interview("interview");
        assert_eq!(interview.role_id, role.id);
        assert_eq!(interview.name, "interview");
    }

    #[test]
    fn test_modify_with_hashmap() {
        let mut role = Role::new(Uuid::new_v4(), "Role name", Timestamp::new(1));
        let original_id = role.id;
        let original_company = role.company_id;

        let mut hash_map: HashMap<String, serde_json::Value> = HashMap::new();
        hash_map.insert("id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("company_id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("name".to_string(), "New name".into());
        hash_map.insert("description".to_string(), "New description".into());
        hash_map.insert("date_applied".to_string(), 3.into());
        hash_map.insert("date_deleted".to_string(), 10.into());

        let partial_role = PartialRole::deserialize(hash_map.into_deserializer()).unwrap();
        role.apply(partial_role);

        assert_eq!(role.id, original_id);
        assert_eq!(role.company_id, original_company);
        assert_eq!(role.name, "New name".to_string());
        assert_eq!(role.description, "New description".to_string());
        assert_eq!(role.date_applied, Timestamp::new(3));
        assert_eq!(role.date_deleted, Some(Timestamp::new(10)));
    }
}
