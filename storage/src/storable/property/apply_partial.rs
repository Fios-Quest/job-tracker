use partially::Partial;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub trait ApplyPartial<'a, P>
where
    Self: Partial<Item = P>,
    P: Deserialize<'a>,
{
    fn apply(&mut self, partial: P) -> bool {
        Partial::apply_some(self, partial)
    }
}

impl<'a, T, P> ApplyPartial<'a, P> for T
where
    T: Partial<Item = P>,
    P: DeserializeOwned,
{
}
