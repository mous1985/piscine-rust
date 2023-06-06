pub struct Message {
    ms: String,
    u: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Self {
        Self { ms, u }
    }

    pub fn send_ms(&self) -> Option<&str> {
        self.ms.as_ref().map(String::as_str)
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(ms) if ms.contains("stupid") => (false, ms),
        Some(ms) => (true, ms),
        None => (false, "empty message"),
    }
}
