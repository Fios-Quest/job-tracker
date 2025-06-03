use uuid::Uuid;

pub trait HasCompany {
    fn get_company_id(&self) -> Uuid;
}
