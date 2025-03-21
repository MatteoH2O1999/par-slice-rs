use par_slice::*;
use std::thread::scope;

#[test]
#[should_panic(expected = "chunk_size should be a divisor of len. 7 / 2 = 3 with a remainder of 1")]
fn invalid_chunk_size() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];

    {
        v.as_par_chunk_index_no_ref(2);
    }
}

//
// Test without threads
//

#[test]
fn no_thread_unchecked() {
    let mut v = vec![1, 2, 3, 4];
    let mut buf = vec![0; 2];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        unsafe {
            slice.get_values_unchecked(0, &mut buf);
        }
        assert_eq!(buf, &[1, 2]);
        unsafe {
            slice.set_values_unchecked(1, &[42, 69]);
        }
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn no_thread_checked() {
    let mut v = vec![1, 2, 3, 4];
    let mut buf = vec![0; 2];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        unsafe {
            slice.get_values(0, &mut buf);
        }
        assert_eq!(buf, &[1, 2]);
        unsafe {
            slice.set_values(1, &[42, 69]);
        }
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
#[should_panic(expected = "Index 42 invalid for slice of len 2")]
fn no_thread_checked_panic_get() {
    let mut v = vec![1, 2, 3, 4];
    let mut buf = vec![0; 2];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        unsafe {
            slice.get_values(42, &mut buf);
        }
    }
}

#[test]
#[should_panic(expected = "Index 69 invalid for slice of len 2")]
fn no_thread_checked_panic_set() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        unsafe {
            slice.set_values(69, &[42, 42]);
        }
    }
}

#[test]
#[should_panic(
    expected = "value should have the same length as the chunk. Got a value of length 1 for a chunk of length 2"
)]
fn no_thread_checked_panic_set_chunk_size() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        unsafe {
            slice.set_values(1, &[42]);
        }
    }
}

//
// Test with a single thread
//

#[test]
fn single_thread_unchecked() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(
                    unsafe { slice.get_values_unchecked(0, vec![0; 2]) },
                    &[1, 2]
                );
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set_values_unchecked(1, &[42, 69]) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn single_thread_checked() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_values(0, vec![0; 2]) }, &[1, 2]);
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set_values(1, &[42, 69]) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn single_thread_checked_panic_get() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                unsafe { slice.get_values(42, vec![0; 2]) };
            })
            .join()
            .unwrap_err();
            s.spawn(|| {
                unsafe { slice.set_values(1, &[42, 69]) };
            })
            .join()
            .unwrap();
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn single_thread_checked_panic_set() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_values(0, vec![0; 2]) }, &[1, 2]);
            })
            .join()
            .unwrap();
            s.spawn(|| {
                unsafe { slice.set_values(69, &[42, 69]) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 3, 4]);
}

//
// Test with multiple threads
//

#[test]
fn multithread_unchecked() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(
                    unsafe { slice.get_values_unchecked(0, vec![0; 2]) },
                    &[1, 2]
                );
            });
            s.spawn(|| {
                unsafe { slice.set_values_unchecked(1, &[42, 69]) };
            });
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn multithread_checked() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_values(0, vec![0; 2]) }, &[1, 2]);
            });
            s.spawn(|| {
                unsafe { slice.set_values(1, &[42, 69]) };
            });
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn multithread_checked_panic_get() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                unsafe { slice.set_values(1, &[42, 69]) };
            });
            s.spawn(|| {
                unsafe { slice.get_values(42, vec![0; 2]) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 42, 69]);
}

#[test]
fn multithread_checked_panic_mut() {
    let mut v = vec![1, 2, 3, 4];

    {
        let slice = v.as_par_chunk_index_no_ref(2);
        scope(|s| {
            s.spawn(|| {
                assert_eq!(unsafe { slice.get_values(0, vec![0; 2]) }, &[1, 2]);
            });
            s.spawn(|| {
                unsafe { slice.set_values(69, &[1, 2]) };
            })
            .join()
            .unwrap_err();
        });
    }

    assert_eq!(v, vec![1, 2, 3, 4]);
}
