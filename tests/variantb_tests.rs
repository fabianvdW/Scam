#[cfg(feature = "variantb")]
mod variantb {
    #[test]
    pub fn test_test() {
        assert_eq!(scam::test(), 43)
    }
}
