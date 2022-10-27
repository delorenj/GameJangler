struct Game {
    title: String,
    location: String,
    platform: Platform
}

struct Platform {
    id: String, //UUID
    name: String,
    location: String
}

impl Game {
    fn new(&mut self) {
        self.title = "No Title".to_string();

    }
}
