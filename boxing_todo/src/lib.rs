pub use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    fn parse(j: JsonValue) -> Option<TodoList> {
        let mut tasks: Vec<Task> = Vec::new();

        match j {
            JsonValue::Object(o) => {
                let title = o.get("title")?.as_str()?;
                if let Some(JsonValue::Array(ts)) = o.get("tasks") {
                    for t in ts {
                        if let JsonValue::Object(o) = t {
                            tasks.push(Task {
                                id: o.get("id")?.as_u32()?,
                                description: o.get("description")?.as_str()?.to_string(),
                                level: o.get("level")?.as_u32()?,
                            });
                        }
                    }
                    return Some(TodoList {
                        title: title.to_string(),
                        tasks,
                    });
                }
            }
            _ => (),
        }

        return None;
    }

    pub fn get_todo(source: &str) -> Result<TodoList, Box<dyn Error>> {
        let json = match read_to_string(source) {
            Err(e) => {
                return Err(Box::new(ReadErr {
                    child_err: Box::new(e),
                }))
            }
            Ok(s) => s,
        };

        match json::parse(&json) {
            Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(e)))),
            Ok(t) => {
                match TodoList::parse(t) {
                    Some(t) => {
                        if t.tasks.len() != 0 {
                            Ok(t)
                        } else {
                            Err(Box::new(ParseErr::Empty))
                        }
                    }
                    None => panic!("This should have never happened; the data was not following the right scheme."),
                }
            }
        }
    }
}
