use crate::prelude::Interview;
use crate::storable::*;
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Role {
    pub id: Uuid,
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

    pub fn create_question<S: Into<String>>(&self, name: S) -> Question {
        Question::new(self, name)
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
    use crate::storable::{
        has_company::test_helper::test_has_company, has_deleted::test_helper::test_has_deleted,
        has_id::test_helper::test_has_id, has_name::test_helper::test_has_name,
    };
    use crate::test_helper::TestHelper;
    use paste::paste;

    test_has_id!(Role);
    test_has_name!(Role);
    test_has_company!(Role);
    test_has_deleted!(Role);

    #[test]
    fn test_create_question() {
        let role = Role::new(Uuid::new_v4(), "role", Timestamp::now());
        let question = role.create_question("question");
        assert_eq!(question.role_id, role.id);
        assert_eq!(question.name, "question");
    }

    #[test]
    fn test_create_interview() {
        let role = Role::new(Uuid::new_v4(), "role", Timestamp::now());
        let interview = role.create_interview("interview");
        assert_eq!(interview.role_id, role.id);
        assert_eq!(interview.name, "interview");
    }
}
