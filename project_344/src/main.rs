/*Does this function need lifetime annotations?
Explain why or why not.
No need lifetime, because the function not return a reference.
*/
fn double_the_length<T>(v: &[T]) -> usize {
    v.len() * 2
}

fn main() {
    // Double len.
    let v = vec![1, 2, 3];
    let dl = double_the_length(&v);
    //let dl = double_the_length(&vec![1, 2, 3]);
    println!("double len: {}", dl);

    // Last two elements.
    let values = vec![1, 2, 3];
    let lt = last_two(&values);
    println!("last two: {:?}", lt);

    println!("first_five: {}", first_five("refrigerator", "Hello"));

    println!(
        "find_string_that_has_content: {}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}

/*
Does this function need lifetime annotations?
Explain why or why not.
Not need lifetime, rust will defer, because there is only one ref arg and one ref returned.
*/

fn last_two<T>(slice: &[T]) -> &[T] {
    if slice.len() >= 2 {
        &slice[slice.len() - 2..]
    } else {
        slice
    }
}

/*

Define a 'first_five' function that accepts two string
slice parameters: 'text' and 'announcement'. The function
should print the value of 'announcement' and return a
slice of the first 5 bytes of 'text'.


Example:
first_five("refrigerator", "Hello") => "refri"

Does this function need lifetime annotations?
Explain why or why not.

Yes, it need lifetime annotations because it get two ref.

--
*/
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("announcement: {announcement}");
    if text.len() >= 5 {
        return &text[..5];
    }
    text
}

/*
Define a `find_string_that_has_content` function that
accepts three string slice parameters: `first`,
`second`, and `target`.
*/

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        return first;
    }
    second
}

/*

Does this function need lifetime annotations?
Explain why or why not.

Yes, because the result ref can be the first or the second param.
*/
