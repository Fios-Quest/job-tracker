use crate::storable::{Flag, HasDeleted, HasId, HasName, Role};
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        Role::new(self.id, name, date_created)
    }

    pub fn create_green_flag<S: Into<String>>(&self, name: S) -> Flag {
        Flag::new_green(self.id, name)
    }

    pub fn create_red_flag<S: Into<String>>(&self, name: S) -> Flag {
        Flag::new_red(self.id, name)
    }
}

impl PartialEq for Company {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HasId for Company {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasName for Company {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasDeleted for Company {
    fn is_deleted(&self) -> bool {
        self.date_deleted.is_some()
    }
}

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
        has_name::test_helper::test_has_name, HasCompany,
    };
    use crate::test_helper::TestHelper;
    use paste::paste;

    test_has_id!(Company);
    test_has_name!(Company);
    test_has_deleted!(Company);

    #[test]
    fn test_partial_eq_same_company() {
        let company1 = Company::new("name".to_string());
        let company2 = company1.clone();
        assert_eq!(company1, company2);
    }

    #[test]
    fn test_partial_eq_same_id() {
        let company1 = Company::new("name".to_string());
        let company2 = Company {
            name: "new name".to_string(),
            ..company1.clone()
        };
        assert_ne!(company1.get_name(), company2.get_name());
        assert_eq!(company1, company2);
    }

    #[test]
    fn test_partial_eq_after_delete() {
        let company1 = Company::new("name".to_string());
        let company2 = Company {
            date_deleted: Some(Timestamp::now()),
            ..company1.clone()
        };
        assert!(!company1.is_deleted());
        assert!(company2.is_deleted());
        assert_eq!(company1, company2);
    }

    #[test]
    fn test_partial_eq_different_id() {
        let company1 = Company::new("name".to_string());
        let company2 = Company::new("name".to_string());
        assert_ne!(company1, company2);
    }

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
