use crate::composite_store::HasFutureStoreFor;
use crate::storable::*;
use crate::storage::*;
use crate::Sealed;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    company_store: Arc<Mutex<C>>,
    flag_store: Arc<Mutex<F>>,
    value_store: Arc<Mutex<V>>,
    role_store: Arc<Mutex<R>>,
    interview_store: Arc<Mutex<I>>,
    question_store: Arc<Mutex<Q>>,
}

impl<C, F, V, R, I, Q> ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    pub fn new(
        company_store: C,
        flag_store: F,
        value_store: V,
        role_store: R,
        interview_store: I,
        question_store: Q,
    ) -> Self {
        Self {
            company_store: Arc::new(Mutex::new(company_store)),
            flag_store: Arc::new(Mutex::new(flag_store)),
            value_store: Arc::new(Mutex::new(value_store)),
            role_store: Arc::new(Mutex::new(role_store)),
            interview_store: Arc::new(Mutex::new(interview_store)),
            question_store: Arc::new(Mutex::new(question_store)),
        }
    }

    pub async fn company_store(&self) -> MutexGuard<C> {
        self.company_store.lock().await
    }

    pub async fn flag_store(&self) -> MutexGuard<F> {
        self.flag_store.lock().await
    }

    pub async fn role_store(&self) -> MutexGuard<R> {
        self.role_store.lock().await
    }

    pub async fn question_store(&self) -> MutexGuard<Q> {
        self.question_store.lock().await
    }

    pub async fn interview_store(&self) -> MutexGuard<I> {
        self.interview_store.lock().await
    }

    pub async fn value_store(&self) -> MutexGuard<V> {
        self.value_store.lock().await
    }
}

impl<C, F, V, R, I, Q> Sealed for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Company> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = C;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.company_store().await
    }
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Flag> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = F;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.flag_store().await
    }
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Role> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = R;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.role_store().await
    }
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Interview> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = I;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.interview_store().await
    }
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Question> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = Q;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.question_store().await
    }
}

impl<C, F, V, R, I, Q> HasFutureStoreFor<Value> for ThreadSafeGeneralStore<C, F, V, R, I, Q>
where
    C: CompanyStore,
    F: FlagStore,
    V: ValueStore,
    R: RoleStore,
    I: InterviewStore,
    Q: QuestionStore,
{
    type Storage = V;

    async fn get_store<'a>(&'a self) -> MutexGuard<'a, Self::Storage>
    where
        Self::Storage: 'a,
    {
        self.value_store().await
    }
}

#[cfg(test)]
mod test_helper {
    use super::*;
    use crate::prelude::Interview;
    use crate::storage::StubStore;
    use crate::test_helper::TestHelper;

    #[cfg(test)]
    impl TestHelper
        for ThreadSafeGeneralStore<
            StubStore<Company>,
            StubStore<Flag>,
            StubStore<Value>,
            StubStore<Role>,
            StubStore<Interview>,
            StubStore<Question>,
        >
    {
        #[cfg(test)]
        async fn new_test() -> anyhow::Result<Self> {
            let store = ThreadSafeGeneralStore::new(
                StubStore::default(),
                StubStore::default(),
                StubStore::default(),
                StubStore::default(),
                StubStore::default(),
                StubStore::default(),
            );
            Ok(store)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{
        recall_by_company::test_helper::test_recall_by_company,
        recall_by_id::test_helper::test_recall_by_id,
        recall_by_name::test_helper::test_recall_by_name,
        recall_by_role::test_helper::test_recall_by_role,
    };
    use crate::test_helper::*;
    use crate::Timestamp;
    use paste::paste;

    test_recall_by_id!(ThreadSafeGeneralStore, Company);
    test_recall_by_id!(ThreadSafeGeneralStore, Flag);
    test_recall_by_id!(ThreadSafeGeneralStore, Value);
    test_recall_by_id!(ThreadSafeGeneralStore, Role);
    test_recall_by_id!(ThreadSafeGeneralStore, Question);
    test_recall_by_id!(ThreadSafeGeneralStore, Interview);
    test_recall_by_name!(ThreadSafeGeneralStore, Company);
    test_recall_by_name!(ThreadSafeGeneralStore, Flag);
    test_recall_by_name!(ThreadSafeGeneralStore, Value);
    test_recall_by_name!(ThreadSafeGeneralStore, Role);
    test_recall_by_name!(ThreadSafeGeneralStore, Question);
    test_recall_by_name!(ThreadSafeGeneralStore, Interview);
    test_recall_by_company!(ThreadSafeGeneralStore, Flag);
    test_recall_by_company!(ThreadSafeGeneralStore, Value);
    test_recall_by_company!(ThreadSafeGeneralStore, Role);
    test_recall_by_role!(ThreadSafeGeneralStore, Question);
    test_recall_by_role!(ThreadSafeGeneralStore, Interview);

    // ---- The following tests are more to show how the API of ThreadSafeGeneralStore ----

    #[tokio::test]
    async fn test_base_store() {
        let company = Company::new("name");
        let flag = Flag::new_green(company.id, "good");
        let role = Role::new(company.id, "role", Timestamp::now());
        let question = Question::new(role.id, "question");

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(company.clone()).await.unwrap();
        all_store.store(flag.clone()).await.unwrap();
        all_store.store(role.clone()).await.unwrap();
        all_store.store(question.clone()).await.unwrap();

        let recalled_company: Company = all_store.recall_by_id(&company).await.unwrap();
        let recalled_flag: Flag = all_store.recall_by_id(&flag).await.unwrap();
        let recalled_role: Role = all_store.recall_by_id(&role).await.unwrap();
        let recalled_question: Question = all_store.recall_by_id(&question).await.unwrap();

        assert_eq!(recalled_company, company);
        assert_eq!(recalled_flag, flag);
        assert_eq!(recalled_role, role);
        assert_eq!(recalled_question, question);
    }

    #[tokio::test]
    async fn test_recall_by_name() {
        let company = Company::new("name");

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(company.clone()).await.unwrap();

        let recalled_companies: Vec<Company> =
            all_store.recall_by_name(&company.name).await.unwrap();

        assert!(recalled_companies.contains(&company));
    }

    #[tokio::test]
    async fn test_recall_by_company() {
        let company = Company::new("name");
        let flag = Flag::new_green(company.id, "good".to_string());
        let role = Role::new(company.id, "role".to_string(), Timestamp::now());

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(flag.clone()).await.unwrap();
        all_store.store(role.clone()).await.unwrap();

        let recalled_flags: Vec<Flag> = all_store.recall_by_company(&company).await.unwrap();
        let recalled_roles: Vec<Role> = all_store.recall_by_company(&company).await.unwrap();

        assert!(recalled_flags.contains(&flag));
        assert!(recalled_roles.contains(&role));
    }

    #[tokio::test]
    async fn test_recall_by_role() {
        let company = Company::new("name");
        let role = Role::new(company.id, "role", Timestamp::now());
        let question = Question::new(role.id, "question");

        let mut all_store = ThreadSafeGeneralStore::new(
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
            StubStore::default(),
        );

        all_store.store(question.clone()).await.unwrap();

        let recalled_questions: Vec<Question> = all_store.recall_by_role(&role).await.unwrap();

        assert!(recalled_questions.contains(&question));
    }
}
