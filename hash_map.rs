use std::collections::HashMap;

fn main() {
    let v = vec![5, 5, 8, 8, 8, 3, 4, 4, 4, 4, 4, 7, 7];
    // let mut hm = HashMap<i32, i32>;
    let mut hm = HashMap::new();
    hm.insert(5, 1);
    let val = hm.get(&5).unwrap();
    println!("val: {val}");
    println!("Contains key 5: {}", hm.contains_key(&5));

    for item in &v {
        // println!("item: {}", *item);
        // Entry return a Entry enum, which implements or_insert().
        // or_insert returns a mutable reference to the value in the entry.
        *hm.entry(*item).or_insert(0) += 1;
    }
    println!("hm: {:?}", hm);
}