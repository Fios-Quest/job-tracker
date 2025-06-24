use crate::storable::{HasCompany, HasDeleted, HasId, HasName};
use crate::{impl_has_company, impl_has_deleted, impl_has_id, impl_has_name, Timestamp};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlagColor {
    Green,
    Red,
}

impl FromStr for FlagColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "green" => Ok(Self::Green),
            "red" => Ok(Self::Red),
            _ => Err(format!("Invalid flag_color '{}'", s)),
        }
    }
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
    pub fn new_green<C: HasId, S: Into<String>>(company: C, name: S) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            flag_color: FlagColor::Green,
            name: name.into(),
            date_deleted: None,
        }
    }

    pub fn new_red<C: HasId, S: Into<String>>(company: C, name: S) -> Self {
        Flag {
            id: Uuid::new_v4(),
            company_id: company.get_id(),
            flag_color: FlagColor::Red,
            name: name.into(),
            date_deleted: None,
        }
    }
}

impl PartialEq for Flag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.company_id == other.company_id
    }
}

impl_has_id!(Flag);
impl_has_name!(Flag);
impl_has_company!(Flag);
impl_has_deleted!(Flag);

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;
    use uuid::Uuid;

    impl TestHelper for Flag {
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Flag::new_green(Uuid::new_v4(), "Green Flag"))
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

    test_has_id!(Flag);
    test_has_name!(Flag);
    test_has_company!(Flag);
    test_has_deleted!(Flag);

    #[test]
    fn test_create_green_flag() {
        let green_flag = Flag::new_green(Uuid::new_v4(), "green flag");
        assert_eq!(green_flag.flag_color, FlagColor::Green);
    }

    #[test]
    fn test_create_red_flag() {
        let red_flag = Flag::new_red(Uuid::new_v4(), "red flag");
        assert_eq!(red_flag.flag_color, FlagColor::Red);
    }

    #[test]
    fn test_flag_color_from_string() {
        use FromStr;

        let flag_color = FlagColor::from_str("red").unwrap();
        assert_eq!(flag_color, FlagColor::Red);
        let flag_color = FlagColor::from_str("RED").unwrap();
        assert_eq!(flag_color, FlagColor::Red);
        let flag_color = FlagColor::from_str("ReD").unwrap();
        assert_eq!(flag_color, FlagColor::Red);

        let flag_color = FlagColor::from_str("green").unwrap();
        assert_eq!(flag_color, FlagColor::Green);
        let flag_color = FlagColor::from_str("GREEN").unwrap();
        assert_eq!(flag_color, FlagColor::Green);
        let flag_color = FlagColor::from_str("gReEn").unwrap();
        assert_eq!(flag_color, FlagColor::Green);

        assert!(FlagColor::from_str("blue").is_err());
    }
}
