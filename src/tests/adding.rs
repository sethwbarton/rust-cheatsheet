mod adding_tests {

    use crate::adding;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(adding::add(3, 4), 7);
    }
}
