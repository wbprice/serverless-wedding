#[derive(Deserialize, Debug)]
pub struct RSVP {
    household_id: String,
    id: String,
    name: String,
    salutation: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

#[derive(Deserialize, Debug)]
pub struct CreateRSVP {
    name: String,
    email_address: String
}