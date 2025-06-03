use crate::storable::{HasCompany, HasDeleted, HasId, HasName};
use crate::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlagColor {
    Green,
    Red,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    pub id: Uuid,
    pub company_id: Uuid,
    pub flag_color: FlagColor,
    pub name: String,
    pub date_deleted: Option<Timestamp>,
}

impl Flag {
    pub fn new_green(company_id: Uuid, name: String) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id,
            flag_color: FlagColor::Green,
            name,
            date_deleted: None,
        }
    }

    pub fn new_red(company_id: Uuid, name: String) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id,
            flag_color: FlagColor::Red,
            name,
            date_deleted: None,
        }
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HasName for Flag {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasId for Flag {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl HasDeleted for Flag {
    fn is_deleted(&self) -> bool {
        self.date_deleted.is_some()
    }
}

impl HasCompany for Flag {
    fn get_company_id(&self) -> Uuid {
        self.company_id
    }
}
