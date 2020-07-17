#[macro_export]
/// uwu!
macro_rules! uwu {
    () => {
        eprintln!("uwu! [{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                eprintln!("uwu! [{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { $crate::uwu!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::uwu!($val)),+,)
    };
}
