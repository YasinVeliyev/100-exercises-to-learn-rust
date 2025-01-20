use crate::status::*;
use crate::{description::*, title::*};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TicketId(pub u64);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ticket {
    id: TicketId,
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

impl Ticket {
    pub fn new(id: TicketId, draft: TicketDraft) -> Self {
        Self {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        }
    }

    pub fn id(&self) -> TicketId {
        self.id
    }
    pub fn title(&self) -> &String {
        self.title.get()
    }

    pub fn description(&self) -> &String {
        self.description.get()
    }
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn set_title(&mut self, title: TicketTitle) {
        self.title = title;
    }

    pub fn set_description(&mut self, description: TicketDescription) {
        self.description = description
    }
    pub fn set_status(&mut self, status: Status) {
        self.status = status
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{:?}",
            self.id.0,
            self.title(),
            self.description(),
            self.status
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketPatch {
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
    pub status: Option<Status>,
}
