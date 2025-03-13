use crate::Timestamp;
use std::cmp::Ordering;
use uuid::Uuid;

pub trait GetName {
    fn get_name(&self) -> &str;
}

pub trait GetId {
    fn get_id(&self) -> Uuid;
}

pub trait GetDeleted {
    fn get_deleted(&self) -> Option<Timestamp>;
}

pub trait SetDeleted {
    fn set_deleted(&mut self, time: Timestamp);
}

pub fn create_name_sort<N: GetName>(partial_name: &str) -> impl Fn(&N, &N) -> Ordering {
    let partial_name = partial_name.to_lowercase();
    move |a, b| {
        let a_name = a.get_name().to_lowercase();
        let b_name = b.get_name().to_lowercase();

        if a_name == b_name {
            Ordering::Equal
        } else if a_name == partial_name {
            Ordering::Less
        } else if b_name == partial_name {
            Ordering::Greater
        } else if a_name.starts_with(&partial_name) && b_name.starts_with(&partial_name) {
            a_name.cmp(&b_name)
        } else if a_name.starts_with(&partial_name) {
            Ordering::Less
        } else if b_name.starts_with(&partial_name) {
            Ordering::Greater
        } else {
            a_name.cmp(&b_name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl GetName for String {
        fn get_name(&self) -> &str {
            &self
        }
    }

    #[test]
    fn test_create_name_sort() {
        let unsorted = vec![
            "Hello".to_string(),
            "No match".to_string(),
            "Hello".to_string(),
            "Hello, world!".to_string(),
            "Hell is other people".to_string(),
            "Still no match".to_string(),
        ];

        let mut sorted = unsorted.clone();

        sorted.sort_by(create_name_sort("Hell"));
        assert_eq!(
            sorted,
            vec![
                "Hell is other people".to_string(),
                "Hello".to_string(),
                "Hello".to_string(),
                "Hello, world!".to_string(),
                "No match".to_string(),
                "Still no match".to_string(),
            ]
        );

        sorted.sort_by(create_name_sort("Hello"));
        assert_eq!(
            sorted,
            vec![
                "Hello".to_string(),
                "Hello".to_string(),
                "Hello, world!".to_string(),
                "Hell is other people".to_string(),
                "No match".to_string(),
                "Still no match".to_string(),
            ]
        );
    }
}
