use asserteq_pretty_macros::impl_tuple;
use seq_macro::seq;

use crate::PrettyDiff;

// Due to a temporary restriction in Rust’s type system, the following traits are only implemented on tuples of arity 12 or less. (https://doc.rust-lang.org/std/primitive.tuple.html)
// (we need Eq and Debug)
seq!(N in 0..=12 {
    impl_tuple!(N);
});
