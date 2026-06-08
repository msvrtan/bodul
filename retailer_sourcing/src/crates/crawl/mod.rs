pub fn info() -> &'static str {
    "crawl module v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_info() {
        assert_eq!(info(), "crawl module v0.1.0");
    }
}
