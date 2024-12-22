use par_slice::*;
use std::thread::scope;

//
// Test without threads
//

#[test]
fn no_thread_unchecked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        assert_eq!(unsafe { slice.get_unchecked(1) }, 2);
        unsafe {
            slice.set_unchecked(2, 42);
        }
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn no_thread_checked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        assert_eq!(unsafe { slice.get(1) }, 2);
        unsafe {
            slice.set(2, 42);
        }
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
#[should_panic(expected = "Index 42 invalid for slice of len 3")]
fn no_thread_checked_panic_get() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        unsafe {
            slice.get(42);
        }
    }
}

#[test]
#[should_panic(expected = "Index 69 invalid for slice of len 3")]
fn no_thread_checked_panic_set() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        unsafe {
            slice.set(69, 42);
        }
    }
}

//
// Test with a single thread
//

#[test]
fn single_thread_unchecked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_unchecked(1) }, 2);
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set_unchecked(2, 42) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn single_thread_checked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get(1) }, 2);
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set(2, 42) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn single_thread_checked_panic_get() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                unsafe { slice.get(42) };
            })
            .join()
            .unwrap_err();
            s.spawn(|| {
                unsafe { slice.set(2, 42) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn single_thread_checked_panic_set() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get(1) }, 2);
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set(69, 42) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 3]);
}

//
// Test with multiple threads
//

#[test]
fn multithread_unchecked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_unchecked(1) }, 2);
            });
            s.spawn(|| {
                unsafe { slice.set_unchecked(2, 42) };
            });
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn multithread_checked() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get(1) }, 2);
            });
            s.spawn(|| {
                unsafe { slice.set(2, 42) };
            });
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn multithread_checked_panic_get() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                unsafe { slice.set(2, 42) };
            });
            s.spawn(|| {
                unsafe { slice.get(42) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 42]);
}

#[test]
fn multithread_checked_panic_mut() {
    let mut v = vec![1, 2, 3];

    {
        let slice = v.as_data_race_par_slice();
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get(1) }, 2);
            });
            s.spawn(|| {
                unsafe { slice.set(69, 42) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 3]);
}
