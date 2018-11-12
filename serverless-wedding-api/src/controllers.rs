use uuid::Uuid;

mod models;

pub fn createRSVP(newRSVP: models::NewRSVP) -> models::modesls {
    models::RSVP {
        household_id: Uuid::new_v4(),
        id: Uuid::new_v4(),
        name: newRSVP["name"],
        email_address: newRSVP["email_address"],
        attending: false,
        invitation_submitted: false,
        reminder_submitted: false
    }
}