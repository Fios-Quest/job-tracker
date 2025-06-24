use crate::storable::{Company, Role};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ApplicationContextError {
    #[error("Company not set")]
    CompanyNotSet,
    #[error("Role does not belong to company")]
    RoleDoesNotBelongToCompany,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationContext {
    company: Option<Arc<Company>>,
    role: Option<Arc<Role>>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            company: None,
            role: None,
        }
    }

    pub fn unset_company(self) -> Self {
        Self::new()
    }

    pub fn unset_role(self) -> Self {
        Self { role: None, ..self }
    }

    pub fn set_company(self, company: Company) -> Self {
        Self {
            role: None,
            company: Some(Arc::new(company)),
        }
    }

    pub fn set_role(self, role: Role) -> Result<Self, ApplicationContextError> {
        match self.company.as_ref().map(|company| company.id) {
            Some(company_id) if company_id == role.company_id => Ok(Self {
                role: Some(Arc::new(role)),
                ..self
            }),
            Some(_) => Err(ApplicationContextError::RoleDoesNotBelongToCompany),
            None => Err(ApplicationContextError::CompanyNotSet),
        }
    }

    pub fn get_company(&self) -> Option<Arc<Company>> {
        self.company.clone()
    }

    pub fn get_role(&self) -> Option<Arc<Role>> {
        self.role.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::TestHelper;
    use crate::Timestamp;

    #[test]
    fn test_new() {
        let context = ApplicationContext::new();
        // Initial company value must not be set
        assert_eq!(context.get_company(), None,);
        // Initial role value must not be set
        assert_eq!(context.get_role(), None,);
    }
    #[tokio::test]
    async fn test_set_get_unset_company_id() {
        let context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test role", Timestamp::now());

        let context_with_company = context.set_company(company.clone());
        assert_eq!(
            context_with_company.get_company(),
            Some(Arc::new(company)),
            "Company value must be set"
        );

        let context_with_role = context_with_company.set_role(role.clone()).unwrap();
        assert_eq!(
            context_with_role.get_role(),
            Some(Arc::new(role)),
            "Role value must be set"
        );

        let context_unset_company = context_with_role.unset_company();
        assert_eq!(
            context_unset_company.get_company(),
            None,
            "Company value must be unset"
        );
        assert_eq!(
            context_unset_company.get_role(),
            None,
            "Role value must be unset"
        );
    }

    #[tokio::test]
    async fn test_set_get_unset_role_id() {
        let context = ApplicationContext::new();

        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test Role", Timestamp::now());

        let context_with_company = context.set_company(company.clone());
        let context_with_role = context_with_company.set_role(role.clone()).unwrap();

        assert_eq!(
            context_with_role.get_role(),
            Some(Arc::new(role)),
            "Role value must be set"
        );

        let context_unset_role = context_with_role.unset_role();
        assert_eq!(
            context_unset_role.get_role(),
            None,
            "Role value must be unset"
        );
    }

    #[tokio::test]
    async fn test_can_not_set_role_id_without_company_id() {
        let context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test Role", Timestamp::now());

        assert_eq!(
            context.set_role(role),
            Err(ApplicationContextError::CompanyNotSet)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_role_id_with_wrong_company_id() {
        let context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = Role::new_test().await.unwrap();
        let context_with_company = context.set_company(company);

        assert_eq!(
            context_with_company.set_role(role),
            Err(ApplicationContextError::RoleDoesNotBelongToCompany)
        );
    }
}
