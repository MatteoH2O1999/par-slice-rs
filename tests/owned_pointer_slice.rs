use par_slice::*;
use std::thread::scope;

//
// Test without threads
//

#[test]
fn no_thread_unchecked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    assert_eq!(unsafe { *slice.get_ptr_unchecked(1) }, 2);
    unsafe {
        *slice.get_mut_ptr_unchecked(2) = 42;
    }

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn no_thread_checked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    assert_eq!(unsafe { *slice.get_ptr(1) }, 2);
    unsafe {
        *slice.get_mut_ptr(2) = 42;
    }

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
#[should_panic(expected = "Index 42 invalid for slice of len 3")]
fn no_thread_checked_panic() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    slice.get_ptr(42);
}

#[test]
#[should_panic(expected = "Index 69 invalid for slice of len 3")]
fn no_thread_checked_panic_mut() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    slice.get_ptr(69);
}

//
// Test with a single thread
//

#[test]
fn single_thread_unchecked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr_unchecked(1) }, 2);
        })
        .join()
        .unwrap();
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr_unchecked(2) = 42 };
        })
        .join()
        .unwrap();
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn single_thread_checked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr(1) }, 2);
        })
        .join()
        .unwrap();
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(2) = 42 };
        })
        .join()
        .unwrap();
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn single_thread_checked_panic() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            slice.get_ptr(42);
        })
        .join()
        .unwrap_err();
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(2) = 42 };
        })
        .join()
        .unwrap();
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn single_thread_checked_panic_mut() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr(1) }, 2);
        })
        .join()
        .unwrap();
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(69) = 42 };
        })
        .join()
        .unwrap_err();
    });

    assert_eq!(slice.into(), vec![1, 2, 3]);
}

//
// Test with multiple threads
//

#[test]
fn multithread_unchecked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr_unchecked(1) }, 2);
        });
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr_unchecked(2) = 42 };
        });
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn multithread_checked() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr(1) }, 2);
        });
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(2) = 42 };
        });
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn multithread_checked_panic() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(2) = 42 };
        });
        s.spawn(|| {
            slice.get_ptr(42);
        })
        .join()
        .unwrap_err();
    });

    assert_eq!(slice.into(), vec![1, 2, 42]);
}

#[test]
fn multithread_checked_panic_mut() {
    let slice = vec![1, 2, 3].into_pointer_par_index();

    scope(|s| {
        s.spawn(|| {
            assert_eq!(unsafe { *slice.get_ptr(1) }, 2);
        });
        s.spawn(|| {
            unsafe { *slice.get_mut_ptr(69) = 42 };
        })
        .join()
        .unwrap_err();
    });

    assert_eq!(slice.into(), vec![1, 2, 3]);
}
