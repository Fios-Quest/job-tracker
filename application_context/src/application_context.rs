use std::sync::Arc;
use storage::prelude::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ApplicationContextError {
    #[error("Company wasn't set")]
    CompanyNotSet,
    #[error("Role wasn't set")]
    RoleNotSet,
    #[error("Role does not belong to the company")]
    RoleDoesNotBelongToCompany,
    #[error("Interview does not belong to the role")]
    InterviewDoesNotBelongToRole,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ApplicationContext {
    company: Option<Arc<Company>>,
    role: Option<Arc<Role>>,
    interview: Option<Arc<Interview>>,
}

impl ApplicationContext {
    pub fn new() -> Self {
        Self {
            company: None,
            role: None,
            interview: None,
        }
    }

    pub fn unset_company(self) -> Self {
        Self::new()
    }

    pub fn unset_role(self) -> Self {
        Self {
            role: None,
            interview: None,
            ..self
        }
    }

    pub fn set_company(self, company: Company) -> Self {
        Self {
            interview: None,
            role: None,
            company: Some(Arc::new(company)),
        }
    }

    pub fn set_role(self, role: Role) -> Result<Self, ApplicationContextError> {
        // Company must set
        let Some(company) = self.get_company() else {
            return Err(ApplicationContextError::CompanyNotSet);
        };
        // Role must be for company
        if company.id != role.company_id {
            return Err(ApplicationContextError::RoleDoesNotBelongToCompany);
        }

        Ok(Self {
            interview: None, // Unset interview when changing role
            role: Some(Arc::new(role)),
            ..self
        })
    }

    pub fn set_interview(self, interview: Interview) -> Result<Self, ApplicationContextError> {
        // Company must be set
        let Some(_company) = self.get_company() else {
            return Err(ApplicationContextError::CompanyNotSet);
        };
        // Role must be set
        let Some(role) = self.get_role() else {
            return Err(ApplicationContextError::RoleNotSet);
        };
        // Interview must be for Role
        if interview.role_id != role.id {
            return Err(ApplicationContextError::InterviewDoesNotBelongToRole);
        }

        Ok(Self {
            interview: Some(Arc::new(interview)),
            ..self
        })
    }

    pub fn get_company(&self) -> Option<Arc<Company>> {
        self.company.clone()
    }

    pub fn get_role(&self) -> Option<Arc<Role>> {
        self.role.clone()
    }

    pub fn get_interview(&self) -> Option<Arc<Interview>> {
        self.interview.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> Timestamp {
        Timestamp::from_timestamp(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_secs() as i64,
        )
    }

    #[test]
    fn test_new() {
        let context = ApplicationContext::new();
        // Nothing should be set when context is created
        assert_eq!(context.get_company(), None);
        assert_eq!(context.get_role(), None);
        assert_eq!(context.get_interview(), None);
    }

    #[tokio::test]
    async fn test_set_get_unset_company() {
        let context = ApplicationContext::new();
        let company = Company::new("Test company");
        let role = company.create_role("Test role", now());

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
    async fn test_set_get_unset_role() {
        let context = ApplicationContext::new();

        let company = Company::new("Test company");
        let role = company.create_role("Test Role", now());
        let interview = role.create_interview("Test Interview");

        let context_with_details = context
            .set_company(company.clone())
            .set_role(role.clone())
            .unwrap()
            .set_interview(interview.clone())
            .unwrap();

        assert_eq!(
            context_with_details.get_role(),
            Some(Arc::new(role)),
            "Role value must be set"
        );

        let context_unset_role = context_with_details.unset_role();
        assert_eq!(
            context_unset_role.get_role(),
            None,
            "Role value must be unset"
        );
    }

    #[tokio::test]
    async fn test_can_not_set_role_without_company() {
        let context = ApplicationContext::new();
        let company = Company::new("Test Company");
        let role = company.create_role("Test Role", now());

        assert_eq!(
            context.set_role(role),
            Err(ApplicationContextError::CompanyNotSet)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_role_with_wrong_company() {
        let context = ApplicationContext::new();
        let company = Company::new("Test Company");
        let another_company_not_set = Company::new("Unused Company");
        let role = another_company_not_set.create_role("Test Role", now());
        let context_with_company = context.set_company(company);

        assert_eq!(
            context_with_company.set_role(role),
            Err(ApplicationContextError::RoleDoesNotBelongToCompany)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_interview_without_company() {
        let context = ApplicationContext::new();
        let unset_company = Company::new("Unused Company");
        let unused_role = unset_company.create_role("Unused role", now());
        let interview = unused_role.create_interview("Test Interview");

        assert_eq!(
            context.set_interview(interview),
            Err(ApplicationContextError::CompanyNotSet)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_interview_without_role() {
        let context = ApplicationContext::new();
        let company = Company::new("Test Company");
        let unused_role = company.create_role("Unused role", now());
        let interview = unused_role.create_interview("Test Interview");

        let context = context.set_company(company);

        assert_eq!(
            context.set_interview(interview),
            Err(ApplicationContextError::RoleNotSet)
        );
    }

    #[tokio::test]
    async fn test_can_not_set_interview_with_wrong_role() {
        let company = Company::new("Test Company");
        let used_role = company.create_role("Used role", now());
        let unused_role = company.create_role("Unused role", now());
        let interview = unused_role.create_interview("Test Interview");
        let context = ApplicationContext::new()
            .set_company(company)
            .set_role(used_role)
            .unwrap();

        assert_eq!(
            context.set_interview(interview),
            Err(ApplicationContextError::InterviewDoesNotBelongToRole)
        );
    }

    #[tokio::test]
    async fn test_setting_role_unsets_interview() {
        let company = Company::new("Test Company");
        let role = company.create_role("Test role", now());
        let interview = role.create_interview("Test interview");
        let context = ApplicationContext::new()
            .set_company(company.clone())
            .set_role(role)
            .unwrap()
            .set_interview(interview.clone())
            .unwrap();

        assert_eq!(context.get_interview().unwrap().as_ref(), &interview);

        let new_role = company.create_role("Test role", now());
        let context = context.set_role(new_role).unwrap();

        assert!(context.get_interview().is_none());
    }
}
