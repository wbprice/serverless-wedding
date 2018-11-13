extern crate uuid;
use uuid::Uuid;

/*
 * Models
 */

#[derive(Deserialize, Serialize, Debug)]
pub struct RSVP {
    household_id: Uuid,
    id: Uuid,
    name: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewRSVP {
    name: String,
    email_address: String
}

/**
 * Methods
 */

pub fn create_rsvp(newRSVP: NewRSVP) -> RSVP {
    RSVP {
        household_id: Uuid::new_v4(),
        id: Uuid::new_v4(),
        name: newRSVP.name,
        email_address: newRSVP.email_address,
        attending: false,
        invitation_submitted: false,
        reminder_submitted: false
    }
}

#[cfg(test)]
mod rsvp_tests {

    use rsvp::{NewRSVP, create_rsvp};

    #[test]
    fn test_create_rsvp() {
        let input = NewRSVP {
            name: "Blaine Price".to_string(),
            email_address: "email@example.com".to_string()
        };
        let result = create_rsvp(input);
        println!("The result was {:?}", result);
    }
}
