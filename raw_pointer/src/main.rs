fn main() {
    let mut sushi = String::from("Yellotail");
    // Imutable raw pointer, two way of declare.
    #[allow(dead_code)]
    let _sushi_raw_pointer_1 = &raw const sushi;
    #[allow(dead_code)]
    let _sushi_raw_pointer_2: *const String = &sushi;
    // Mutable raw pointer.
    let _sushi_raw_mutable_pointer_1 = &raw mut sushi;
    #[allow(dead_code)]
    let _sushi_raw_mutable_pointer_2: *mut String = &raw mut sushi;
    let _sushi_raw_mutable_pointer_3: *mut String = &mut sushi;

    // Will cause raw pointer to pointer to undeterminated value.
    drop(sushi);

    unsafe {
        println!("_sushi_raw_pointer_1: {}", *_sushi_raw_mutable_pointer_1);
    }
}
