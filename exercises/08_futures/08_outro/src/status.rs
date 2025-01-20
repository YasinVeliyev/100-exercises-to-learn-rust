use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        let value = value.to_lowercase();
        if value == "todo" {
            return Ok(Self::ToDo);
        } else if value == "inprogress" {
            return Ok(Self::InProgress);
        } else if value == "done" {
            return Ok(Self::Done);
        }
        Err("Only `todo`,`inprogress`,  `done`".to_owned())
    }
}
