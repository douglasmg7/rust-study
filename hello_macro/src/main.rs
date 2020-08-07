// macro_rules! hello {
// ( $( $e: expr ),* ) => {
// {
// $(
// println!("Olá {}", $e);
// )*
// }
// };
// }

// stmt.execute_named(&[(":code", &product.code), (":technical_description", &product.technical_description)]).unwrap();

struct Stmt {}

impl Stmt {
    pub fn execute_named(&self, entry: (String, String)) {
        let (a, b) = entry;
        println!("a = {}, b = {}", a, b);
    }
}

macro_rules! my_print {
    ($i: ident, $e: expr) => {
        let $i = {
            let a = $e;
            println!("{}", a);
            a
        };
    };
}

macro_rules! populate_execute {
    ($f: ident, $a: expr, $b: expr) => {
        $f.execute_named(($a.to_string(), $b.to_string()));
    };
}

fn main() {
    // hello!("asdf", "fdas", "qwer");
    my_print!(x, 30 + 12);
    println!("Again: {}", x);

    let st = Stmt {};

    // st.execute_named(("asdf".to_string(), "qwer".to_string()));
    populate_execute!(st, "wert", "erty");
}
