use crate::storable::*;
use crate::{impl_has_deleted, impl_has_id, impl_has_name, impl_has_role, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub role_id: Uuid,
    pub name: String,
    pub answer: String,
    pub date_deleted: Option<Timestamp>,
}

impl Question {
    pub fn new<R: HasId, S: Into<String>>(role: R, name: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            role_id: role.get_id(),
            name: name.into(),
            answer: "".to_string(),
            date_deleted: None,
        }
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.role_id == other.role_id
    }
}

impl_has_id!(Question);
impl_has_name!(Question);
impl_has_role!(Question);
impl_has_deleted!(Question);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Question {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Question::new(Uuid::new_v4(), "Question"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storable::{
        has_deleted::test_helper::test_has_deleted, has_id::test_helper::test_has_id,
        has_name::test_helper::test_has_name, has_role::test_helper::test_has_role,
    };
    use crate::test_helper::TestHelper;
    use paste::paste;

    test_has_id!(Question);
    test_has_name!(Question);
    test_has_role!(Question);
    test_has_deleted!(Question);
}
