use crate::PrettyDiff;

impl<T: PrettyDiff> PrettyDiff for Option<T> {
    fn pretty_diff(left: &Self, right: &Self) -> String {
        match (left,right) {
            (Some(a),Some(b)) => format!("Option contents difference: {}",PrettyDiff::pretty_diff(a,b)),
            (Some(_),None) => "Some != None".to_owned(),
            (None,Some(_)) => "None != Some".to_owned(),
            (None,None) => panic!(""),
        }
    }
}