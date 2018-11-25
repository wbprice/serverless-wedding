use std::env;
use uuid::Uuid;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput, PutItemError};

/*
 * Models
 */

#[derive(Debug, Clone, Hash, Serialize)]
pub struct RSVP {
    household_id: String,
    id: String,
    name: String,
    email_address: String,
    attending: bool,
    invitation_submitted: bool,
    reminder_submitted: bool
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct NewRSVP {
    name: String,
    email_address: String
}

/**
 * Methods
 */

pub fn create_rsvp(new_rsvp: NewRSVP) -> RSVP {
    RSVP {
        household_id: Uuid::new_v4().to_string().into(),
        id: Uuid::new_v4().to_string(),
        name: new_rsvp.name.into(),
        email_address: new_rsvp.email_address.into(),
        attending: false.into(),
        invitation_submitted: false.into(),
        reminder_submitted: false.into()
    }
}

pub fn create_rsvp_record(new_rsvp: NewRSVP) -> Result<PutItemOutput, PutItemError>{
    let rsvp : RSVP = create_rsvp(new_rsvp);
    let client = DynamoDbClient::new(Region::UsEast1);
    let table_name = env::var("RSVP_TABLE_ARN").is_err().to_string();

    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&rsvp).unwrap(),
        table_name: table_name.into(),
        ..PutItemInput::default()
    };
    
    match client.put_item(input).sync() {
        Ok(output) => {
            return Ok(output);
        },
        Err(err) => {
            return Err(err);
        }
    }
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
