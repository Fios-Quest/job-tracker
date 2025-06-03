use uuid::Uuid;

pub trait HasCompany {
    fn get_company_id(&self) -> Uuid;
}

impl<T> HasCompany for &T
where
    T: HasCompany,
{
    fn get_company_id(&self) -> Uuid {
        (*self).get_company_id()
    }
}
