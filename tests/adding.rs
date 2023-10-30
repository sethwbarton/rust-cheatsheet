use rust_cheatsheet::adding::add;

#[test]
fn adds_two_numbers() {
    assert_eq!(add(3, 4), 7);
}
