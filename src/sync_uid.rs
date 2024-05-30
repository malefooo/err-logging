use once_cell::sync::Lazy;
use std::cell::RefCell;
use uuid::Uuid;

// Define a thread local storage to store UUID
thread_local! {
    static SYNC_THREAD_UUID: RefCell<Lazy<String>> = RefCell::new(Lazy::new(||{Uuid::new_v4().to_string()}));
}

pub fn sync_get() -> String {
    SYNC_THREAD_UUID.with(|uid| uid.borrow().clone())
}

#[cfg(test)]
mod test {
    use crate::sync_uid::sync_get;
    use std::thread;

    #[test]
    fn test_concurrent_get() {
        for _ in 0..5 {
            thread::spawn(|| {
                let uid = sync_get();
                println!("uid: {uid}");
            })
            .join()
            .unwrap();
        }
    }
}
