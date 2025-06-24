use crate::storable::{HasCompany, HasDeleted, HasId, HasName};
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: String,
    pub date_applied: Timestamp,
    pub date_deleted: Option<Timestamp>,
}

impl Role {
    pub fn new<S: Into<String>>(company_id: Uuid, name: S, date_applied: Timestamp) -> Role {
        Role {
            id: Uuid::new_v4(),
            company_id,
            name: name.into(),
            description: "".to_string(),
            date_applied,
            date_deleted: None,
        }
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.company_id == other.company_id
    }
}

impl HasId for Role {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasCompany for Role {
    fn get_company_id(&self) -> Uuid {
        self.company_id
    }
}

impl HasName for Role {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasDeleted for Role {
    fn is_deleted(&self) -> bool {
        self.date_deleted.is_some()
    }
}

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
}
