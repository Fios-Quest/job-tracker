use crate::storable::object::role::Role;
use crate::storable::property::has_deleted::HasDeleted;
use crate::storable::property::has_id::HasId;
use crate::storable::property::has_name::HasName;
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub date_deleted: Option<Timestamp>,
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

impl Company {
    pub fn new<S: Into<String>>(name: S) -> Company {
        Company {
            id: Uuid::new_v4(),
            name: name.into(),
            date_deleted: None,
        }
    }

    pub fn create_role(&self, name: String, date_created: Timestamp) -> Role {
        Role::new(self.id, name, date_created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
