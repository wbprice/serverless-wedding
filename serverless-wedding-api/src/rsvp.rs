
use uuid::Uuid;
use dynomite::{dynamodb, DynamoDbExt, FromAttributes, Item};

/*
 * Models
 */

#[derive(Item, Debug, Clone, Deserialize, Serialize)]
pub struct RSVP {
    household_id: String,
    #[hash]
    id: String,
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
        household_id: Uuid::new_v4().to_string(),
        id: Uuid::new_v4().to_string().into(),
        name: new_rsvp.name,
        email_address: new_rsvp.email_address.into(),
        attending: false,
        invitation_submitted: false,
        reminder_submitted: false
    }
}

pub fn create_rsvp_record(new_rsvp: NewRSVP) -> RSVP {
    create_rsvp(new_rsvp)
}


#[cfg(test)]
mod rsvp_tests {

    use rsvp::{NewRSVP, create_rsvp, create_rsvp_record};

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

    #[test]
    fn test_create_rsvp_record() {
        let result = create_rsvp_record(NewRSVP {
            name: "Blaine Price".to_string(),
            email_address: "email@example.com".to_string()
        });
        println!("{:?}", result);
    }
}
