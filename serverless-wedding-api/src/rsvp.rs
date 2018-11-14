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

pub fn create_rsvp(new_rsvp: NewRSVP) -> RSVP {
    RSVP {
        household_id: Uuid::new_v4(),
        id: Uuid::new_v4(),
        name: new_rsvp.name,
        email_address: new_rsvp.email_address,
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

        let result = create_rsvp(NewRSVP {
            name: "Blaine Price".to_string(),
            email_address: "email@example.com".to_string()
        });

        assert_eq!(result.name, "Blaine Price".to_string());
        assert_eq!(result.email_address, "email@example.com".to_string());
        assert_eq!(result.attending, false);
        assert_eq!(result.invitation_submitted, false);
        assert_eq!(result.reminder_submitted, false);
    }
}
