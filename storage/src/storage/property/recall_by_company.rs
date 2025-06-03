use crate::storable::property::has_company::HasCompany;
use anyhow::Result;

pub trait RecallByCompany<T>
where
    T: HasCompany + Clone,
{
    async fn recall_by_company<C: HasCompany>(&self, company: C) -> Result<Vec<T>>;
}
