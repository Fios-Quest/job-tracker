use partially::Partial;
use serde::de::DeserializeOwned;

pub trait ApplyPartial<P>
where
    Self: Partial<Item = P>,
    P: DeserializeOwned,
{
    fn apply(&mut self, partial: P) -> bool {
        Partial::apply_some(self, partial)
    }
}

impl<T, P> ApplyPartial<P> for T
where
    T: Partial<Item = P>,
    P: DeserializeOwned,
{
}
