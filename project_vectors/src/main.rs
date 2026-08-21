#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: vec![],
        }
    }
    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }
    fn delete_file(&mut self, index: usize) -> File {
        self.contents.remove(index)
    }
    fn get_file(&mut self, index: usize) -> Option<&File> {
        self.contents.get(index)
    }
}

fn main() {
    let mut folder = Folder::new("linux".to_string());
    folder.create_file(String::from("some_file_1.txt"));
    folder.create_file(String::from("some_file_2.txt"));
    println!("{folder:#?}");

    folder.delete_file(0);
    println!("{folder:#?}");

    match folder.get_file(0) {
        Some(file) => println!("{file:#?}"),
        None => println!("There is no file"),
    }

    match folder.get_file(1) {
        Some(file) => println!("{file:#?}"),
        None => println!("There is no file"),
    }
}
