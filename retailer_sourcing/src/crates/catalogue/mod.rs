pub fn info() -> &'static str {
    "catalogue module v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_info() {
        assert_eq!(info(), "catalogue module v0.1.0");
    }
}
