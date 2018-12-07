use std::env;
use std::collections::HashMap;
use uuid::Uuid;
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, AttributeValue, PutItemInput, PutItemError, QueryInput, QueryOutput, QueryError};
use serde_dynamodb;

/*
 * Models
 */

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
        household_id: Uuid::new_v4().to_string().into(),
        id: Uuid::new_v4().to_string(),
        name: new_rsvp.name.into(),
        email_address: new_rsvp.email_address.into(),
        attending: false.into(),
        invitation_submitted: false.into(),
        reminder_submitted: false.into()
    }
}

pub fn create_rsvp_record(new_rsvp: NewRSVP) -> Result<RSVP, PutItemError> {
    let rsvp : RSVP = create_rsvp(new_rsvp);
    let client = DynamoDbClient::new(Region::UsEast1);

    let input = PutItemInput {
        item: serde_dynamodb::to_hashmap(&rsvp).unwrap(),
        table_name: env::var("RSVP_TABLE_NAME").unwrap(),
        ..PutItemInput::default()
    };
    
    match client.put_item(input).sync() {
        Ok(_) => {
            return Ok(rsvp);
        },
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn list_household_rsvps(household_id: String) -> Result<QueryOutput, QueryError> {
    let client = DynamoDbClient::new(Region::UsEast1);

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(String::from(":household_id"), AttributeValue {
        s: Some(String::from(household_id)),
        ..Default::default()
    });

    let input = QueryInput {
        key_condition_expression: Some("household_id = :household_id".to_string()),
        expression_attribute_values: Some(expression_attribute_values),
        table_name: env::var("RSVP_TABLE_NAME").unwrap(),
        ..Default::default()
    };

    match client.query(input).sync() {
        Ok(response) => {
            return Ok(response);
        },
        Err(err) => {
            return Err(err);
        }
    }
}


#[cfg(test)]
mod rsvp_tests {

    use rsvp::{NewRSVP, create_rsvp, create_rsvp_record, list_household_rsvps};

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

    #[test]
    fn test_list_household_rsvps() {
        let result = list_household_rsvps("2e12f811-67c5-4b48-ac9c-1c6777421235".to_string());
        println!("{:?}", result);
    }
}
