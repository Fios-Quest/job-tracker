use crate::storable::{HasCompany, HasId};
use anyhow::Result;

pub trait RecallByCompany<T>
where
    T: HasCompany + Clone,
{
    async fn recall_by_company<I: HasId>(&self, company_id: &I) -> Result<Vec<T>>;
}
