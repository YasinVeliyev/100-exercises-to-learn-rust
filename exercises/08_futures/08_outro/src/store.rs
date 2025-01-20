use crate::ticket::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
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

    pub fn update(&mut self, id: TicketId, patch: TicketPatch) -> Result<bool, String> {
        let TicketPatch {
            title,
            description,
            status,
        } = patch;
        let mut updated = false;
        match self.get_mut(id) {
            Some(ticket) => {
                if let Some(title) = title {
                    ticket.set_title(title);
                    updated = true;
                }
                if let Some(description) = description {
                    ticket.set_description(description);
                    updated = true;
                }
                if let Some(status) = status {
                    ticket.set_status(status);
                    updated = true;
                }
                return Ok(updated);
            }
            None => Err("Ticket not found".to_owned()),
        }
    }

    pub fn delete(&mut self, id: TicketId) -> bool {
        match self.tickets.remove(&id) {
            Some(t) => true,
            None => false,
        }
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
