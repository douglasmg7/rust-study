//Let's model a real-time chat system where users can
//share audio and video files.

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

//Add an impl block for ChatMessage structs whose T type
//is a DigitalContent enum. Define a `consume_entertainment`
//method that prints out the value of the `content` field in
//Debug format. For example, "Watching the AudioFile".
impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content)
    }
}

//Add an impl block for ChatMessage structs with any type T.
//Define a `retrieve_time` method that returns a String.
//It should return a clone of the `time` field from
//the struct.
impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    //In `main`, create a ChatMessage with `content` set to a
    //string slice.
    let msg_1 = ChatMessage {
        content: "Olá",
        time: "09:28:19".to_string(),
    };
    //Create another ChatMessage with `content` set to a String.
    let msg_2 = ChatMessage {
        content: "Lá vai".to_string(),
        time: "09:31:30".to_string(),
    };

    //Create another ChatMessage with `content' set to a
    //DigitalContent variant.
    let msg_3 = ChatMessage {
        content: DigitalContent::VideoFile,
        time: "09:32:39".to_string(),
    };

    let msg_4 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "09:37:06".to_string(),
    };

    //Invoke the `consume_entertainment` method on the
    //ChatMessage storing a DigitalContent enum.
    msg_3.consume_entertainment();

    //Invoke the `retrieve_time` method on all 3 ChatMessage
    //instances and print out each String's content.
    println!("msg_1 time: {}", msg_1.retrieve_time());
    println!("msg_2 time: {}", msg_2.retrieve_time());
    println!("msg_3 time: {}", msg_3.retrieve_time());
    println!("msg_4 time: {}", msg_4.retrieve_time());
}
