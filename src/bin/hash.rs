use std::collections::HashMap;

fn main() {
    let mut names = HashMap::new();
    let new = names.insert(1_u32, "Liz");
    assert_eq!(new, None);

    let old = names.insert(1_u32, "Pat");
    assert_eq!(old, Some("Liz"));

    assert_eq!(names[&1], "Pat");
    assert_eq!(names.get(&1), Some(&"Pat"));
    assert_eq!(names.get(&11), None);
}
