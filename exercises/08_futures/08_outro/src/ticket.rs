use crate::{description::*, title::*};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TicketId(u64);

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
    pub fn title(&self) -> &TicketTitle {
        &self.title
    }

    pub fn description(&self) -> &TicketDescription {
        &self.description
    }
    pub fn status(&self) -> Status {
        self.status
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket::new(id, ticket);
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Index<&TicketId> for TicketStore {
    type Output = Ticket;

    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}
