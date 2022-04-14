use std::sync::{Arc, Mutex};

fn boxcar_wrap_mutex_primitive() {
    let vec = Arc::new(boxcar::Vec::new());

    // insert an element
    vec.push(Mutex::new(1));

    const THREAD_NUM: i32 = 1000;
    let mut thread_join_handles = Vec::with_capacity(THREAD_NUM as usize);
    for _idx in 0..THREAD_NUM {
        thread_join_handles.push(std::thread::spawn({
            let vec = vec.clone();
            move || {
                // mutate through the mutex
                *vec[0].lock().unwrap() += 1;
            }
        }));
    }

    for thread_join_handle in thread_join_handles {
        let _ = thread_join_handle.join();
    }
    let x = vec[0].lock().unwrap();
    assert_eq!(*x, 1001);
}

fn boxcar_wrap_mutex_vec() {
    let vec = Arc::new(boxcar::Vec::new());

    // insert an element
    vec.push(Mutex::new(Vec::new()));

    const THREAD_NUM: i32 = 1000;
    let mut thread_join_handles = Vec::with_capacity(THREAD_NUM as usize);
    for idx in 0..THREAD_NUM {
        thread_join_handles.push(std::thread::spawn({
            let vec = vec.clone();
            move || {
                // mutate through the mutex
                vec[0].lock().unwrap().push(idx);
            }
        }));
    }

    for thread_join_handle in thread_join_handles {
        let _ = thread_join_handle.join();
    }
    let v = vec[0].lock().unwrap();
    assert_eq!(1000 as usize, v.len());
}

fn boxcar_wrap_boxcar() {
    let vec = Arc::new(boxcar::Vec::new());

    // insert an element
    vec.push(boxcar::Vec::new());

    const THREAD_NUM: i32 = 1000;
    let mut thread_join_handles = Vec::with_capacity(THREAD_NUM as usize);
    for idx in 0..THREAD_NUM {
        thread_join_handles.push(std::thread::spawn({
            let vec = vec.clone();
            move || {
                // mutate through the mutex
                vec[0].push(idx);
            }
        }));
    }

    for thread_join_handle in thread_join_handles {
        let _ = thread_join_handle.join();
    }
    assert_eq!(THREAD_NUM as usize, vec[0].len());
}

#[cfg(test)]
mod tests {
    use crate::boxcar_wrap_boxcar;
    use crate::boxcar_wrap_mutex_primitive;
    use crate::boxcar_wrap_mutex_vec;
    #[test]
    fn it_works() {
        boxcar_wrap_mutex_primitive();
        boxcar_wrap_mutex_vec();
        boxcar_wrap_boxcar();
        assert_eq!(1, 1);
    }
}
