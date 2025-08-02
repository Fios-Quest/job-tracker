use dioxus::html::FormData;
use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::collections::HashMap;
use storage::prelude::ApplyPartial;

pub trait ModifyWithFormData<'a, P>
where
    Self: ApplyPartial<'a, P>,
    P: Deserialize<'a>,
{
    fn modify_with_form_data(&mut self, form_data: &FormData) -> anyhow::Result<()> {
        let data: HashMap<_, serde_json::Value> = form_data
            .values()
            .into_iter()
            .map(|(k, v)| (k, v.as_value()))
            .filter(|(_k, v)| !v.is_empty())
            .map(|(k, v)| (k, v.into()))
            .collect();

        let partial = P::deserialize(data.into_deserializer())?;
        self.apply(partial);

        Ok(())
    }
}

impl<'a, T, P> ModifyWithFormData<'a, P> for T
where
    T: ApplyPartial<'a, P>,
    P: Deserialize<'a>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::html::{FormValue, SerializedFormData};
    use storage::prelude::Company;
    use storage::Timestamp;

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
