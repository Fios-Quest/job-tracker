use crate::storable::object::role::Role;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq)]
struct CompanyContext {
    id: Uuid,
    role: Option<Role>,
}

impl CompanyContext {
    #[must_use]
    fn new(id: Uuid) -> Self {
        Self { id, role: None }
    }

    #[must_use]
    fn set_role(self, role: Role) -> Self {
        Self {
            id: self.id,
            role: Some(role),
        }
    }

    #[must_use]
    fn unset_role(self) -> Self {
        Self {
            id: self.id,
            role: None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ApplicationContextError {
    CompanyNotSet,
}

impl fmt::Display for ApplicationContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationContextError::CompanyNotSet => write!(f, "Company is not set"),
        }
    }
}

impl std::error::Error for ApplicationContextError {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationContext {
    company_context: Option<CompanyContext>,
}

impl ApplicationContext {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn get_company_id(&self) -> Option<Uuid> {
        self.company_context.as_ref().map(|c| c.id)
    }

    #[must_use]
    pub fn set_company_id(self, id: Uuid) -> Self {
        Self {
            company_context: Some(CompanyContext::new(id)),
        }
    }

    #[must_use]
    pub fn unset_company_id(self) -> Self {
        Self {
            company_context: None,
        }
    }

    #[must_use]
    pub fn get_role(&self) -> Option<&Role> {
        self.company_context.as_ref().and_then(|c| c.role.as_ref())
    }

    pub fn set_role(self, role: Role) -> Result<Self, ApplicationContextError> {
        // Only works if a company exists in the context
        self.company_context
            .map(|company_context| Self {
                company_context: Some(company_context.set_role(role)),
            })
            .ok_or(ApplicationContextError::CompanyNotSet)
    }

    #[must_use]
    pub fn unset_role_id(self) -> Self {
        Self {
            company_context: self.company_context.map(|c| c.unset_role()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Timestamp;

    #[test]
    fn test_new() {
        let context = ApplicationContext::new();
        assert_eq!(
            context.get_company_id(),
            None,
            "Initial company value must not be set"
        );
        assert_eq!(
            context.get_role(),
            None,
            "Initial role value must not be set"
        );
    }
    #[test]
    fn test_set_get_unset_company_id() {
        let context = ApplicationContext::new();
        let company_id = Uuid::new_v4();
        let role = Role::new(company_id, "Test Role".to_string(), Timestamp::now());

        let context_with_company_id = context.set_company_id(company_id);
        assert_eq!(
            context_with_company_id.get_company_id(),
            Some(company_id),
            "Company value must be set"
        );

        let context_with_role = context_with_company_id.set_role(role.clone()).unwrap();
        assert_eq!(
            context_with_role.get_role(),
            Some(&role),
            "Role value must be set"
        );

        let context_unset_company_id = context_with_role.unset_company_id();
        assert_eq!(
            context_unset_company_id.get_company_id(),
            None,
            "Company value must be unset"
        );
        assert_eq!(
            context_unset_company_id.get_role(),
            None,
            "Role value must be unset"
        );
    }

    #[test]
    fn test_set_get_unset_role_id() {
        let context = ApplicationContext::new();

        let company_id = Uuid::new_v4();
        let role = Role::new(company_id, "Test Role".to_string(), Timestamp::now());

        let context_with_company_id = context.set_company_id(company_id);
        let context_with_role_id = context_with_company_id.set_role(role.clone()).unwrap();

        assert_eq!(
            context_with_role_id.get_role(),
            Some(&role),
            "Role value must be set"
        );

        let context_unset_company_id = context_with_role_id.unset_role_id();
        assert_eq!(
            context_unset_company_id.get_role(),
            None,
            "Role value must be unset"
        );
    }

    #[test]
    fn test_can_not_set_role_id_without_company_id() {
        let context = ApplicationContext::new();
        let company_id = Uuid::new_v4();
        let role = Role::new(company_id, "Test Role".to_string(), Timestamp::now());

        let set_role_result = context.set_role(role);
        assert_eq!(set_role_result, Err(ApplicationContextError::CompanyNotSet));
        assert_eq!(
            set_role_result.unwrap_err().to_string(),
            "Company is not set".to_string()
        );
    }
}
