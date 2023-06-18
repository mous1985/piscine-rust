#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link =Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment{
            grade:None
        }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(worker));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(mut boxed_worker) = self.grade.take() {
            let worker_name = boxed_worker.name.clone();
            self.grade = boxed_worker.next.take();
            Some(worker_name)
        } else {
            None
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        self.grade.as_ref().map(|worker| (worker.name.clone(), worker.role.clone()))
    }
}
