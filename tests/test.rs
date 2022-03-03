
#[cfg(test)]
mod test {
    use tutorial::collections::vector;

    #[test]
    fn is_3_odd() {
        assert_eq!((3 - 1) % 2, 0);
    }

    #[test]
    fn is_3_not_even() {
        assert_ne!(3 % 2, 0);
    }

    #[test]
    #[should_panic]
    fn is_going_to_panic() {
        panic!("Paniced!");
    }

    #[test]
    fn is_vector_not_empty() {
        assert!(vector::create_dummy_vector().len() > 0);
    }

    #[test]
    fn is_result_useful() -> Result<(), String> {
        Ok(())
    }

    #[test]
    #[ignore]
    fn is_file_writable() {
        assert!(true);
    }
}