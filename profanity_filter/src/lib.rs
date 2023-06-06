pub struct Message {
    ms: String,
    u: String,
}

impl Message {
    pub fn new(ms: String, u: String) -> Self {
        Self { ms, u }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.ms.is_empty() {
            None
        } else {
            Some(&self.ms)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        Some(ms) if ms.contains("stupid") => (false, ms),
        Some(ms) => (true, ms),
        None => (false, "empty message"),
    }
}
