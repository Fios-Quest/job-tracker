use crate::storable::*;
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Interview {
    pub id: Uuid,
    pub role_id: Uuid,
    pub name: String,
    pub notes: String,
    pub host: Vec<String>,
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
            host: Vec::with_capacity(0),
            date_time: None,
            date_deleted: None,
        }
    }
}

impl_has_id!(Interview);
impl_has_name!(Interview);
impl_has_role!(Interview);
impl_has_deleted!(Interview);

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
    use crate::storable::{
        has_deleted::test_helper::test_has_deleted, has_id::test_helper::test_has_id,
        has_name::test_helper::test_has_name, has_role::test_helper::test_has_role,
    };
    use crate::test_helper::TestHelper;
    use paste::paste;

    test_has_id!(Interview);
    test_has_name!(Interview);
    test_has_role!(Interview);
    test_has_deleted!(Interview);
}
