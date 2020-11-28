#[cfg(feature = "varianta")]
mod varianta {
    #[test]
    pub fn test_test() {
        assert_eq!(scam::test(), 42)
    }
}
