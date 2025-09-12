//// Traits

trait Process<T: Channel> {
    fn process(&self, channel: T) -> T;
}

trait Channel: std::marker::Copy {
    type Part;

    fn get<T>(&self, part: Part) -> T;
    fn set<T>(&self, part: Part, value: T) -> T;
}

//// Channels

#[derive(Clone, Copy)]
struct GraphicsChannel {
    input: (isize, isize),
    output: Rgb,
}

enum GraphicsPart {
    Input,
    Output,
}

impl Channel<Part = GraphicsPart> for GraphicsChannel {
    fn get<(isize, isize)>(&self, part: Part) -> T {
        match part {
            Self::Part::Input => self.input,
            _ => panic!("Waaa");
        }
    }
    fn get<Rgb>(&self, part: Part) -> T {
        match part {
            Self::Part::Input => self.input,
            _ => panic!("Waaa");
        }
    }
}

#[derive(Clone, Copy)]
struct EventChannel {
    input: Event,
    output: Event,
}

//// Processes

struct Player {
    location: (isize, isize),
    sprite: Box<dyn Process<GraphicsChannel>>,
}

impl Process<GraphicsChannel> for Player {
    fn process(&self, channel: GraphicsChannel) -> GraphicsChannel {
        self.t
        self.sprite.process(channel)
    }
}

impl Process<EventChannel> for Player {
    fn process(&self, channel: EventChannel) -> EventChannel {
        if let Some(key) = channel.keyboard_event {
            
        }
    }
}

//// Helpers

#[derive(Clone, Copy)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Clone, Copy)]
struct Event {
    keyboard_input: Option<char>,
}

struct Move<T> {
    amount: (isize, isize),
    part: T,
}

impl<T: Channel> Process<T> for Move<T::Part> {
    fn process(&self, channel: T) -> T {
        let (x, y) = channel.get(self.part);
        let (amount_x, amount_y) = self.amount;
        let new_point = (x + amount_x, y + amount_y);
        channel.set(self.part, new_point)
    }
}
