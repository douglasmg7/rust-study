fn main() {
    let csv = "
        Manufactory, Model, Year
        Chevrolet, Monza, 1990
        Fiat, Uno, 1992
    ";

    let mut table: Vec<Vec<String>> = Vec::new();
    for line in csv.lines().map(str::trim).filter(|l| !l.is_empty()) {
        // println!("line: {line:?}");
        table.push(line.split(',').map(str::trim).map(String::from).collect());
    }
    println!("table: {:?}", table);
}
