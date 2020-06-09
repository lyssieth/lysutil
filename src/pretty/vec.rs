//! Prettifies Vec-s

use std::fmt::{Debug, Display};
use string_builder::Builder;

/// Returns a given Vec<T> as a String, prettified.
///
/// # Arguments
///
/// * `v` - a Vec<T: Display> to prettify
///
/// # Remarks
///
/// There is a `pretty_vec_debug` for items that implement Debug, but not Display
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// use lysutil::pretty::pretty_vec;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let vec = vec![1, 5, 2, 44, 18272]; // Contents of the Vec must implement std::fmt::Display
/// let pretty = pretty_vec(vec);
/// #   Ok(())
/// # }
/// ```
pub fn pretty_vec<T>(v: Vec<T>) -> String
where
    T: Display,
{
    if v.len() > 1 {
        let mut b = Builder::default();
        b.append("[");
        let mut iters = 0;
        for x in &v {
            if iters < v.len() - 1 {
                b.append(format!("{}, ", *x));
                iters += 1;
            } else {
                b.append(format!("{}", *x));
            }
        }

        b.append("]");
        b.string().unwrap()
    } else if v.len() == 1 {
        format!("[{}]", v[0])
    } else {
        format!("[]")
    }
}

/// Returns a given Vec<T> as a String, prettified.
///
/// # Arguments
///
/// * `v` - a Vec<T: Debug> to prettify
///
/// # Remarks
///
/// There is a `pretty_vec` for items that implement Display, but not Debug
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// use lysutil::pretty::pretty_vec_debug;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let vec = vec![1, 5, 2, 44, 18272]; // Contents of the Vec must implement std::fmt::Debug
/// let pretty = pretty_vec_debug(vec);
/// #   Ok(())
/// # }
/// ```
pub fn pretty_vec_debug<T>(v: Vec<T>) -> String
where
    T: Debug,
{
    if v.len() > 1 {
        let mut b = Builder::default();
        b.append("[");
        let mut iters = 0;
        for x in &v {
            if iters < v.len() - 1 {
                b.append(format!("{:?}, ", *x));
                iters += 1;
            } else {
                b.append(format!("{:?}", *x));
            }
        }

        b.append("]");
        b.string().unwrap()
    } else if v.len() == 1 {
        format!("[{:?}]", v[0])
    } else {
        format!("[]")
    }
}
