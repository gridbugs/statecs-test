use generated::{
    generated_0_a,
    generated_0_b,
};

#[test]
fn component_set_single_bitfield() {
    let mut set = generated_0_a::ComponentSet::new();

    assert!(set.is_empty());

    set.insert_a();

    assert!(!set.is_empty());
    assert!(set.contains_a());
    assert!(!set.contains_b());

    set.insert_b();
    assert!(set.contains_b());

    set.remove_a();
    assert!(!set.contains_a());
    assert!(set.contains_b());

    set.clear();
    assert!(!set.contains_b());
    assert!(set.is_empty());
}

#[test]
fn component_set_multi_bitfield() {
    let mut set = generated_0_b::ComponentSet::new();

    assert!(set.is_empty());

    set.insert_a();

    assert!(!set.is_empty());
    assert!(set.contains_a());
    assert!(!set.contains_b());

    set.insert_b();
    assert!(set.contains_b());

    set.remove_a();
    assert!(!set.contains_a());
    assert!(set.contains_b());

    set.clear();
    assert!(!set.contains_b());
    assert!(set.is_empty());

}
