use std::fmt::Debug;

#[macro_export]
macro_rules! assert_eq_pretty {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::panic_ne(&*left_val, &*right_val, Option::None);
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::panic_ne(&*left_val, &*right_val, Some(core::format_args!($($arg)+)));
                }
            }
        }
    };
}

pub fn panic_ne<T>(left: &T, right: &T, args: Option<std::fmt::Arguments<'_>>) -> !
where
    T: PrettyDiff + Sized,
{
    match args {
        Some(args) => panic!(
            r#"assertion failed: {}: {}"#,
            PrettyDiff::pretty_diff(left, right),
            args
        ),
        None => panic!(
            r#"assertion failed: {}"#,
            PrettyDiff::pretty_diff(left, right)
        ),
    }
}

pub trait PrettyDiff: Debug + PartialEq {
    fn pretty_diff(left: &Self, right: &Self) -> String;
}

pub mod scalar;
pub mod tuple;
pub mod option;