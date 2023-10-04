pub mod config;
pub mod short_classes;
pub mod starter;
pub mod visit_selectors;

#[cfg(test)]
mod tests {
    use crate::starter::start;
    #[test]
    fn t() {
        start();
    }
}
