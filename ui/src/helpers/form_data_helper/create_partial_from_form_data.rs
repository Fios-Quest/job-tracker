use dioxus_html::FormData;
use serde::de::{DeserializeOwned, IntoDeserializer};
use std::collections::HashMap;
use storage::prelude::*;

pub trait CreatePartialFromFormData
where
    Self: DeserializeOwned,
{
    fn from_form_data(form_data: &FormData) -> anyhow::Result<Self> {
        let data: HashMap<_, serde_json::Value> = form_data
            .values()
            .into_iter()
            .map(|(k, v)| (k, v.as_value()))
            .filter(|(_k, v)| !v.is_empty())
            .map(|(k, v)| (k, v.into()))
            .collect();

        Ok(Self::deserialize(data.into_deserializer())?)
    }
}

impl CreatePartialFromFormData for PartialCompany {}
impl CreatePartialFromFormData for PartialFlag {}
impl CreatePartialFromFormData for PartialInterview {}
impl CreatePartialFromFormData for PartialQuestion {}
impl CreatePartialFromFormData for PartialRole {}
impl CreatePartialFromFormData for PartialValue {}
