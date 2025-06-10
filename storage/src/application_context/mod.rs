use crate::prelude::Company;
use crate::storable::Role;
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

    pub fn unset_company(&mut self) {
        self.role = None;
        self.company = None;
    }

    pub fn unset_role(&mut self) {
        self.role = None;
    }

    pub fn set_company(&mut self, company: Company) {
        self.role = None;
        self.company = Some(Arc::new(company));
    }

    pub fn set_role(&mut self, role: Role) -> Result<(), ApplicationContextError> {
        match self.company.as_ref().map(|company| company.id) {
            Some(company_id) if company_id == role.company_id => {
                self.role = Some(Arc::new(role));
                Ok(())
            }
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
        assert_eq!(
            context.get_company(),
            None,
            "Initial company value must not be set"
        );
        assert_eq!(
            context.get_role(),
            None,
            "Initial role value must not be set"
        );
    }
    #[tokio::test]
    async fn test_set_get_unset_company_id() {
        let mut context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test role", Timestamp::now());

        context.set_company(company.clone());
        assert_eq!(
            context.get_company(),
            Some(Arc::new(company)),
            "Company value must be set"
        );

        context.set_role(role.clone()).unwrap();
        assert_eq!(
            context.get_role(),
            Some(Arc::new(role)),
            "Role value must be set"
        );

        context.unset_company();
        assert_eq!(context.get_company(), None, "Company value must be unset");
        assert_eq!(context.get_role(), None, "Role value must be unset");
    }

    #[tokio::test]
    async fn test_set_get_unset_role_id() {
        let mut context = ApplicationContext::new();

        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test Role", Timestamp::now());

        context.set_company(company.clone());
        context.set_role(role.clone()).unwrap();

        assert_eq!(
            context.get_role(),
            Some(Arc::new(role)),
            "Role value must be set"
        );

        context.unset_role();
        assert_eq!(context.get_role(), None, "Role value must be unset");
    }

    #[tokio::test]
    async fn test_can_not_set_role_id_without_company_id() {
        let mut context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = company.create_role("Test Role", Timestamp::now());

        assert_eq!(
            context.set_role(role),
            Err(ApplicationContextError::CompanyNotSet)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_role_id_with_wrong_company_id() {
        let mut context = ApplicationContext::new();
        let company = Company::new_test().await.unwrap();
        let role = Role::new_test().await.unwrap();
        context.set_company(company);

        assert_eq!(
            context.set_role(role),
            Err(ApplicationContextError::RoleDoesNotBelongToCompany)
        );
    }
}
