#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: i32,
    duration_secs: i32,
}

impl TaylorSwiftSong {
    // Associated function - self.
    fn new(title: String, release_year: i32, duration_secs: i32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }

    // fn display_song_info(self: Self) {
    // fn display_song_info(self) {
    // fn display_song_info(self: &Self) {
    fn display_song_info(&self) {
        println!(
            "Title: {}, release_year: {}, duration_secs: {}",
            self.title, self.release_year, self.duration_secs,
        );
    }

    // fn double_length(mut self: Self) {
    // fn double_length(mut self) {
    fn double_length(&mut self) {
        self.duration_secs *= self.duration_secs;
        println!("{:?}", self);
    }
}

fn main() {
    let mut ts = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };
    ts.display_song_info();
    ts.double_length();
    println!("Song title: {}", ts.title);

    let new_song = TaylorSwiftSong::new(String::from("Super Ultra"), 2021, 45);
    dbg!(new_song);
}
