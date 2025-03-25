use std::fmt;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct CompanyContext {
    id: Uuid,
    role: Option<Uuid>,
}

impl CompanyContext {
    #[must_use]
    fn new(id: Uuid) -> Self {
        Self { id, role: None }
    }

    #[must_use]
    fn set_role(self, role: Uuid) -> Self {
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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
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
        self.company_context.map(|c| c.id)
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
    pub fn get_role_id(&self) -> Option<Uuid> {
        self.company_context.and_then(|c| c.role)
    }

    pub fn set_role_id(self, id: Uuid) -> Result<Self, ApplicationContextError> {
        // Only works if a company exists in the context
        self.company_context
            .map(|company_context| Self {
                company_context: Some(company_context.set_role(id)),
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

    #[test]
    fn test_new() {
        let context = ApplicationContext::new();
        assert_eq!(
            context.get_company_id(),
            None,
            "Initial company value must not be set"
        );
        assert_eq!(
            context.get_role_id(),
            None,
            "Initial role value must not be set"
        );
    }
    #[test]
    fn test_set_get_unset_company_id() {
        let context = ApplicationContext::new();
        let company_id = Uuid::new_v4();

        let context_with_company_id = context.set_company_id(company_id);
        assert_eq!(
            context_with_company_id.get_company_id(),
            Some(company_id),
            "Company value must be set"
        );

        let context_unset_company_id = context_with_company_id.unset_company_id();
        assert_eq!(
            context_unset_company_id.get_company_id(),
            None,
            "Company value must be unset"
        );
    }

    #[test]
    fn test_set_get_unset_role_id() {
        let context = ApplicationContext::new();
        let role_id = Uuid::new_v4();

        let context_with_company_id = context.set_company_id(Uuid::new_v4());
        let context_with_role_id = context_with_company_id.set_role_id(role_id).unwrap();

        assert_eq!(
            context_with_role_id.get_role_id(),
            Some(role_id),
            "Company value must be set"
        );

        let context_unset_company_id = context_with_role_id.unset_company_id();
        assert_eq!(
            context_unset_company_id.get_company_id(),
            None,
            "Company value must be unset"
        );
    }

    #[test]
    fn test_can_not_set_role_id_without_company_id() {
        let context = ApplicationContext::new();
        let role_id = Uuid::new_v4();

        assert_eq!(
            context.set_role_id(role_id),
            Err(ApplicationContextError::CompanyNotSet)
        );
    }
}
