pub fn version() -> &'static str {
    "app module v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_version() {
        assert_eq!(version(), "app module v0.1.0");
    }
}
