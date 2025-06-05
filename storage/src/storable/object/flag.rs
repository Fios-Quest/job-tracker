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

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::{TestCounter, TestHelper};
    use uuid::Uuid;

    impl TestHelper for Flag {
        async fn new_test() -> anyhow::Result<Self> {
            const TEST_COUNTER: TestCounter = TestCounter::new();
            let next = TEST_COUNTER.next();
            match next % 2 == 0 {
                true => Ok(Flag::new_green(
                    Uuid::new_v4(),
                    format!("Green Flag {}", next / 2),
                )),
                false => Ok(Flag::new_red(
                    Uuid::new_v4(),
                    format!("Red Flag {}", next / 2),
                )),
            }
        }
    }
}
