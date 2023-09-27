use zcrab::types::DynResult;

#[test]
fn test_dyn_result() {
    use std::fmt;
    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl std::error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> DynResult<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    assert_eq!(double_first(numbers).ok(), Some(84));
    assert!(double_first(empty.clone()).err().unwrap().is::<EmptyVec>());
    if let Some(e) = double_first(empty).err().unwrap().downcast_ref::<EmptyVec>() {
        println!("{:?}", e);
    }
    assert!(double_first(strings)
        .err()
        .unwrap()
        .is::<std::num::ParseIntError>());

    let empty = vec![];
    assert!(!double_first(empty)
        .err()
        .unwrap()
        .is::<std::num::ParseIntError>());
}
