#[cfg(test)]
mod tests {
    use super::ser::to_wxf_bytes_option;
    use super::de::from_wxf_bytes;
    use super::Expr;

    #[test]
    fn option_none_serializes_to_none_symbol() {
        let bytes = to_wxf_bytes_option(None).unwrap();
        // Should decode to Expr::Symbol("None")
        let expr = from_wxf_bytes(&bytes).unwrap();
        assert_eq!(expr, Expr::Symbol("None".to_string()));
    }

    #[test]
    fn option_some_serializes_normally() {
        let bytes = to_wxf_bytes_option(Some(&Expr::integer(42))).unwrap();
        let expr = from_wxf_bytes(&bytes).unwrap();
        assert_eq!(expr, Expr::Integer(42));
    }
}
