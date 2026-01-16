use crate::helpers::{log_error, report_if_error};
use dioxus::core::{spawn, Callback};
use dioxus_html::FormEvent;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use storage::prelude::{ApplyPartial, BaseStore, HasId};
use storage::Partial;

pub fn edit_with_form<O, P, S>(
    store: S,
    storable: Arc<O>,
    callback: Callback<O>,
) -> impl FnMut(FormEvent)
where
    O: Partial<Item = P> + Clone + HasId + 'static,
    P: DeserializeOwned + 'static,
    S: BaseStore<O> + Clone + 'static,
{
    move |e: FormEvent| {
        e.prevent_default();
        if let Ok(form_data) = e.parsed_values::<P>().map_err(log_error) {
            let mut storable = O::clone(&storable);
            let mut store = store.clone();
            spawn(async move {
                storable.apply(form_data);
                report_if_error!(store.store(storable.clone()).await);
                callback(storable);
            });
        }
    }
}
