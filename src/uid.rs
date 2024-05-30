use std::cell::RefCell;
use once_cell::sync::Lazy;
use uuid::Uuid;

// Define a thread local storage to store UUID
thread_local! {
    static THREAD_UUID: RefCell<Lazy<String>> = RefCell::new(Lazy::new(||{Uuid::new_v4().to_string()}));
}

pub fn get() -> String {
    THREAD_UUID.with(|uid|{uid.borrow().clone()})
}

#[cfg(test)]
mod test {
    use std::thread;
    use crate::uid::get;

    #[test]
    fn test_concurrent_get() {
        for _ in 0..5 {
            thread::spawn(||{
                let uid = get();
                println!("uid: {uid}");
            }).join().unwrap();
        }
    }

}