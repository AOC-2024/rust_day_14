use day_14::count_safety_factor;

#[test]
fn should_compute_safety_factor() {
    assert_eq!(count_safety_factor("test/resources/puzzle.txt"), 12);
}