pub mod catalogue;
pub mod collection;
pub mod crawl;
pub mod scheduling;
pub mod types;

pub fn info() -> &'static str {
    "crates module v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_info() {
        assert_eq!(info(), "crates module v0.1.0");
    }
}
