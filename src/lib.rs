pub fn merong() -> String {
    "merong".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn is_same_merong() {
        assert_eq!(merong(), "merong");
    }
}
