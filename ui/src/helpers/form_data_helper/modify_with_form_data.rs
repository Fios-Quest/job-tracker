use super::CreatePartialFromFormData;
use dioxus::html::FormData;
use storage::prelude::*;

pub trait ModifyWithFormData<P>
where
    Self: ApplyPartial<P>,
    P: CreatePartialFromFormData,
{
    fn modify_with_form_data(&mut self, form_data: &FormData) -> anyhow::Result<()> {
        let partial = P::create_partial_from_form_data(form_data)?;
        self.apply(partial);
        Ok(())
    }
}

impl<T, P> ModifyWithFormData<P> for T
where
    T: ApplyPartial<P>,
    P: CreatePartialFromFormData,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus_html::{FormValue, SerializedFormData};
    use std::collections::HashMap;

    fn hash_map_to_form_data<K, V>(hash_map: &HashMap<&K, &V>) -> FormData
    where
        K: ToString + ?Sized,
        V: ToString + ?Sized,
    {
        let form_values = hash_map
            .iter()
            .map(|(k, v)| (k.to_string(), FormValue(vec![v.to_string()])))
            .collect();
        FormData::from(SerializedFormData::new("".to_string(), form_values))
    }

    #[test]
    fn test_modify_company() {
        let mut company = Company::new("Test company");

        let mut hash_map = HashMap::new();
        hash_map.insert("name", "New name");
        hash_map.insert("date_deleted", "2025-07-28T00:00");

        let company_form = hash_map_to_form_data(&hash_map);

        company.modify_with_form_data(&company_form).unwrap();

        assert_eq!(company.name, "New name".to_string());
        assert_eq!(
            company.date_deleted,
            Some(Timestamp::from_string("2025-07-28T00:00"))
        );
    }
}
