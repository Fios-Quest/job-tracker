use crate::storable::*;
use crate::storage::*;
use crate::StorageError;

#[derive(Clone)]
pub struct StubStore<T> {
    store: Vec<T>,
}

impl<T> Default for StubStore<T> {
    fn default() -> Self {
        StubStore { store: Vec::new() }
    }
}

impl<O> BaseStore<O> for StubStore<O>
where
    O: HasId + Clone,
{
    async fn store(&mut self, storable: O) -> anyhow::Result<()> {
        // Remove the item if its already stored
        self.store
            .retain(|stored_item| storable.get_id() != stored_item.get_id());

        // Store the new item
        self.store.push(storable);

        Ok(())
    }
}

impl<O> RecallById<O> for StubStore<O>
where
    O: HasId + HasDeleted + Clone,
{
    async fn recall_by_id<I: HasId>(&self, id: I) -> anyhow::Result<O> {
        Ok(self
            .store
            .iter()
            .filter(|item| !item.is_deleted())
            .find(|stored_item| id.get_id() == stored_item.get_id())
            .cloned()
            .ok_or(StorageError::NotFound)?)
    }
}

impl<T> RecallByName<T> for StubStore<T>
where
    T: HasName + HasDeleted + Clone,
{
    async fn recall_by_name<N: AsRef<str>>(&self, name: N) -> anyhow::Result<Vec<T>> {
        let search_string = name.as_ref().to_lowercase();
        Ok(self
            .store
            .iter()
            .filter(|stored_item| {
                stored_item
                    .get_name()
                    .to_lowercase()
                    .contains(&search_string)
            })
            .filter(|item| !item.is_deleted())
            .cloned()
            .collect())
    }
}

impl<T> RecallByCompany<T> for StubStore<T>
where
    T: HasCompany + HasDeleted + Clone,
{
    async fn recall_by_company<C: HasId>(&self, company: C) -> anyhow::Result<Vec<T>> {
        Ok(self
            .store
            .iter()
            .filter(|stored_item| stored_item.get_company_id() == company.get_id())
            .filter(|item| !item.is_deleted())
            .cloned()
            .collect())
    }
}

impl<T> RecallByRole<T> for StubStore<T>
where
    T: HasRole + HasDeleted + Clone,
{
    async fn recall_by_role<I: HasId>(&self, role: I) -> anyhow::Result<Vec<T>> {
        Ok(self
            .store
            .iter()
            .filter(|stored_item| stored_item.get_role_id() == role.get_id())
            .filter(|item| !item.is_deleted())
            .cloned()
            .collect())
    }
}

impl CompanyStore for StubStore<Company> {}
impl RoleStore for StubStore<Role> {}
impl FlagStore for StubStore<Flag> {}
impl QuestionStore for StubStore<Question> {}
impl InterviewStore for StubStore<Interview> {}
impl ValueStore for StubStore<Value> {}

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::test_helper::TestHelper;

    #[cfg(test)]
    impl<O> TestHelper for StubStore<O>
    where
        O: HasId + Clone,
    {
        #[cfg(test)]
        async fn new_test() -> anyhow::Result<Self> {
            Ok(Self::default())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::property::recall_by_id::test_helper::test_recall_by_id;
    use crate::storage::recall_by_role::test_helper::test_recall_by_role;
    use crate::storage::{
        recall_by_company::test_helper::test_recall_by_company,
        recall_by_name::test_helper::test_recall_by_name,
    };
    use crate::test_helper::*;
    use paste::paste;

    test_recall_by_id!(StubStore, Company);
    test_recall_by_id!(StubStore, Flag);
    test_recall_by_id!(StubStore, Role);
    test_recall_by_id!(StubStore, Value);
    test_recall_by_id!(StubStore, Question);
    test_recall_by_id!(StubStore, Interview);
    test_recall_by_name!(StubStore, Company);
    test_recall_by_name!(StubStore, Flag);
    test_recall_by_name!(StubStore, Role);
    test_recall_by_name!(StubStore, Value);
    test_recall_by_name!(StubStore, Question);
    test_recall_by_name!(StubStore, Interview);
    test_recall_by_company!(StubStore, Flag);
    test_recall_by_company!(StubStore, Role);
    test_recall_by_company!(StubStore, Value);
    test_recall_by_role!(StubStore, Question);
    test_recall_by_role!(StubStore, Interview);
}
