use crate::storable::*;
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Company {
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
    use crate::storable::{
        has_deleted::test_helper::test_has_deleted, has_id::test_helper::test_has_id,
        has_name::test_helper::test_has_name,
    };
    use crate::test_helper::TestHelper;
    use paste::paste;

    test_has_id!(Company);
    test_has_name!(Company);
    test_has_deleted!(Company);

    #[test]
    fn test_create_role() {
        let company = Company::new("company".to_string());
        let role = company.create_role("role".to_string(), Timestamp::now());
        assert_eq!(company.get_id(), role.get_company_id());
    }

    #[test]
    fn test_create_green_flag() {
        let company = Company::new("company".to_string());
        let flag = company.create_green_flag("green flag".to_string());
        assert_eq!(company.get_id(), flag.get_company_id());
    }

    #[test]
    fn test_create_red_flag() {
        let company = Company::new("company".to_string());
        let flag = company.create_red_flag("red flag".to_string());
        assert_eq!(company.get_id(), flag.get_company_id());
    }
}
