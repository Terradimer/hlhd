macro_rules! query_guard {
    ($query:expr) => {
        if let Ok(result) = $query {
            result
        } else {
            return;
        }
    }
}

pub(crate) use query_guard;