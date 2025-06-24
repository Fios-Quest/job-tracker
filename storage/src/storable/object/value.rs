use crate::storable::*;
use crate::{impl_has_company, impl_has_deleted, impl_has_id, impl_has_name, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: String,
    pub date_deleted: Option<Timestamp>,
}

impl Value {
    pub fn new<C: HasId, S: Into<String>>(company: C, name: S) -> Self {
        Self {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            name: name.into(),
            description: "".to_string(),
            date_deleted: None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.company_id == other.company_id
    }
}

impl_has_id!(Value);
impl_has_name!(Value);
impl_has_company!(Value);
impl_has_deleted!(Value);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Value {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Value::new(Uuid::new_v4(), "Value"))
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

    test_has_id!(Value);
    test_has_name!(Value);
    test_has_company!(Value);
    test_has_deleted!(Value);
}
