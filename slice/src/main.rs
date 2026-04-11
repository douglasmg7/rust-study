fn main() {
    // It is possible because &str is write in the binary, and have scope to begin until the end program life time.
    // With String it will not work. Will be dangle memory.
    let text_slice = {
        let text = "Era uma vez";
        &text[0..3]
    };

    dbg!(text_slice);
    // text.push_str(" asdf");
    // dbg!(text);

    let text = String::from("Some text");
    let text_slice2 = &text[4..];
    dbg!(text_slice2);

    // One character can use more than one byte.
    let mut text = "asdf";
    dbg!(text.len());

    text = "ásdf";
    dbg!(text.len());

    // let text2 = &text[..1];      Will not work, because separate the character à.
    let text2 = &text[..2];
    dbg!(text2.len());
    dbg!(text2);

    let ar = [1, 2, 3, 4, 5, 6];
    let ar_ref = &ar; // Not a slice, just a reference.
    let ar_slice = &ar[..3];

    print_len(&ar);
    print_len(ar_ref);
    print_len(ar_slice);
}

// fn print_len(ar: [i32, 6})   Only accept array of 6 elements.
// Accept an array slice.
fn print_len(ar: &[i32]) {
    dbg!(ar.len());
}
