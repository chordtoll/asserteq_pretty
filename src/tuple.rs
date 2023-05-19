use crate::PrettyDiff;

impl<T:PrettyDiff> PrettyDiff for (T,) {
    fn pretty_diff(left: &Self, right: &Self) -> String {
        format!("`({:?})` != `({:?})`",left,right)
    }
}

impl<T:PrettyDiff, U:PrettyDiff> PrettyDiff for (T,U) {
    fn pretty_diff(left: &Self, right: &Self) -> String {
        format!("Differences:{}{}",
            if left.0 != right.0 {
                format!("\n\tPos 0: `{:?}` != `{:?}`",left.0,right.0)
            } else {
                String::new()
            },
            if left.1 != right.1 {
                format!("\n\tPos 1: `{:?}` != `{:?}`",left.1,right.1)
            } else {
                String::new()
            },
        )
    }
}