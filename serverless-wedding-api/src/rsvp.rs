use serde_derive::{{Serialize, Deserialize}};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSVP {
    household_id: String,
    id: String,
    name: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRSVP {
    name: String,
    email_address: String
}

impl RSVP {
    pub fn new(new_rsvp : NewRSVP) -> RSVP {
        RSVP {
            household_id: Uuid::new_v4().to_string().into(),
            id: Uuid::new_v4().to_string(),
            name: new_rsvp.name,
            email_address: new_rsvp.email_address,
            attending: false.into(),
            invitation_submitted: false.into(),
            reminder_submitted: false.into()
        }
    }
}

#[cfg(test)]
mod rsvp_tests {

    use super::*;

    #[test]
    fn test_rsvp_new() {
        let result = RSVP::new(NewRSVP {
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