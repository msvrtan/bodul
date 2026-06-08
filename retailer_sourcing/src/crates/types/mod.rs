pub fn info() -> &'static str {
    "types module v0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_info() {
        assert_eq!(info(), "types module v0.1.0");
    }
}
