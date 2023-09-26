pub mod checker;
pub mod short_tailwind;

mod tests {
    use crate::checker::parse;
    use std::fs::read_to_string;

    #[test]
    fn a() {
        if let Ok(f) = read_to_string("input.css") {
            parse(f, true, true, true);

            assert!(false);
        }
    }
}
