Answer: 3121
Difficulty: 1

# Hint

A value is dropped when it no longer has an owner.

# Explanation

The program prints `3121`. That is, the `Drop` impl for `let _guard = Guard`
runs at the end of main but the `Drop` impl for `let _ = Guard` runs right away.

In general, a value is dropped when it no longer has an owner. The variable
`_guard` owns the first value of type `Guard` and remains in scope until the end
of main. The `_` is not a variable but a wildcard pattern that binds nothing;
since no variables are bound on this line, there is no variable to be the owner
of the second value of type `Guard` and that value is dropped on the same line.

This distinction between the underscore pattern vs variables with a leading
underscore is incredibly important to remember when working with lock guards in
unsafe code.

    use std::sync::Mutex;

    static MUTEX: Mutex<()> = Mutex::new(());

    /// MUTEX must be held when accessing this value.
    static mut VALUE: usize = 0;

    fn main() {
        let _guard = MUTEX.lock().unwrap();
        unsafe {
            VALUE += 1;
        }
    }

If this code were to use `let _ = MUTEX.lock().unwrap()` then the mutex guard
would be dropped immediately, releasing the mutex and failing to guard the
access of `VALUE`.
