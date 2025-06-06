use anyhow::Result;

pub trait TestHelper: Sized {
    async fn new_test() -> Result<Self>;
}
