use iter::{Status, Ticket, TicketStore};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

fn main() {
    let mut store = TicketStore::new();

    let ticket = Ticket {
        title: ticket_title(),
        description: ticket_description(),
        status: Status::ToDo,
    };
    store.add_ticket(ticket);

    let ticket = Ticket {
        title: ticket_title(),
        description: ticket_description(),
        status: Status::InProgress,
    };
    store.add_ticket(ticket);

    for ticket in store {
        println!("{:?}", ticket);
    }
}
