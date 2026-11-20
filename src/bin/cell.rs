use std::cell::Cell;

fn main() {
    let cell = Cell::new(1);
    let ref_cell = &cell;
    // No mut cell can have inner value changed.
    cell.set(2);

    println!("cell: {}", cell.get());
    println!("cell: {}", ref_cell.get());

    ref_cell.set(5);
    println!("cell: {}", cell.get());
    println!("cell: {}", ref_cell.get());

    println!("cell: {}", cell.into_inner());
    // panic!
    //println!("cell: {}", cell.get());
}
