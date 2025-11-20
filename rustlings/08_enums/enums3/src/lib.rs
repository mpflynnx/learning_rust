// My solution to Rustlings enums3.rs
// with comments
// https://github.com/rust-lang/rustlings/blob/main/exercises/08_enums/enums3.rs

// defining struct 'Point'
struct Point {
    x: u64,
    y: u64,
}

// defining enum 'Message'
enum Message {
    Resize { width: u64, height: u64 }, // has named fields like a Struct
    Move(Point), // value is a Point struct
    Echo(String), // includes a single String
    ChangeColor(u8, u8, u8), // includes three u8 values
    Quit, // no data associated with it
}

// defining struct 'State'
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

// defining methods
impl State {

    // State Struct methods, Like Setters, sets the value of the Struct
    // All methods must have 'self' of type 'Self' as their first
    // parameter.
    // '&mut self' is short for 'self: &mut Self'
    fn resize(self: &mut Self, width: u64, height: u64) { 
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    // Takes 'Message' enum as an argument
    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.

        // if Message enum variant set to Resize then
        // pass the enums parameters to Struct 'state' method resize() updates Struct 
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move (point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(red ,green, blue ) => self.change_color(red, green, blue),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {

        // Creating instance of Struct 'State'
        let mut state = State { // instance named 'state'
            width: 0, // specify concrete value using key:value pairs
            height: 0,
            position: Point { x: 0, y: 0 }, // Creating instance Struct 'Point'
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        // Using Struct State method 'process' to mutate the instance named 'state'
        // passes Message Enums variant Resize a struct
        // process() then mutates the Structs width and height fields
        state.process(Message::Resize {
            width: 10,
            height: 30,
        });

        // passes Message Enums variant Move a Point Struct
        state.process(Message::Move(Point { x: 10, y: 15 })); 
        // passes Message Enum variant Echo a String
        state.process(Message::Echo(String::from("Hello world!")));
        // passes Message Enum variant Echo a String
        state.process(Message::ChangeColor(255, 0, 255));
        // activates Message Enum variant Quit       
        state.process(Message::Quit);

        // Tests for state mutations
        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
