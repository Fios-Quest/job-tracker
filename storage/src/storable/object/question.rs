use crate::storable::*;
use crate::Timestamp;
use partially::Partial;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Partial)]
#[partially(derive(Deserialize, Default))]
pub struct Question {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub role_id: Uuid,
    pub name: String,
    pub answer: String,
    pub date_deleted: Option<Timestamp>,
}

impl Question {
    pub fn new<R: HasId, N: Into<String>, A: Into<String>>(role: R, name: N, answer: A) -> Self {
        Self {
            id: Uuid::new_v4(),
            role_id: role.get_id(),
            name: name.into(),
            answer: answer.into(),
            date_deleted: None,
        }
    }
}

impl_has_id!(Question);
impl_has_name!(Question);
impl_has_role!(Question);
impl_has_deleted!(Question);

impl_is_partial_complete_optional_name_only!(PartialQuestion);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Question {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Question::new(Uuid::new_v4(), "Question", "Answer"))
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

    test_has_id!(Question);
    test_has_name!(Question);
    test_has_role!(Question);
    test_has_deleted!(Question);

    #[test]
    fn test_modify_with_hashmap() {
        let mut question = Question::new(Uuid::new_v4(), "Original Question", "Original Answer");
        let original_id = question.id;
        let original_role = question.role_id;

        let mut hash_map: HashMap<String, serde_json::Value> = HashMap::new();
        hash_map.insert("id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("role_id".to_string(), Uuid::new_v4().to_string().into());
        hash_map.insert("name".to_string(), "New Question".into());
        hash_map.insert("answer".to_string(), "New Answer".into());
        hash_map.insert("date_deleted".to_string(), "2025-07-28T00:00".into());

        let partial_question = PartialQuestion::deserialize(hash_map.into_deserializer()).unwrap();
        question.apply(partial_question);

        assert_eq!(question.id, original_id);
        assert_eq!(question.role_id, original_role);
        assert_eq!(question.name, "New Question".to_string());
        assert_eq!(question.answer, "New Answer".to_string());
        assert_eq!(
            question.date_deleted,
            Some(Timestamp::from_string("2025-07-28T00:00"))
        );
    }

    #[test]
    fn test_partial_question_is_complete_complete_question() {
        let question = PartialQuestion {
            name: Some("Test question".to_string()),
            answer: None,
            date_deleted: None,
        };
        assert!(question.is_complete())
    }

    #[test]
    fn test_partial_question_is_complete_missing_name() {
        let question = PartialQuestion {
            name: None,
            answer: None,
            date_deleted: None,
        };
        assert!(!question.is_complete())
    }

    #[test]
    fn test_partial_question_is_complete_empty_name() {
        let question = PartialQuestion {
            name: Some(String::new()),
            answer: None,
            date_deleted: None,
        };
        assert!(!question.is_complete())
    }
}
