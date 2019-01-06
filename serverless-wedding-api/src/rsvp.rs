use serde_derive::{{Serialize, Deserialize}};
use std::vec::{{Vec}};
use std::collections::{{HashMap}};
use std::env;
use uuid::Uuid;
use std::option::Option;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, PutRequest, DynamoDbClient, PutItemInput, WriteRequest, BatchWriteItemInput, BatchWriteItemError};
use serde_dynamodb;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    email_address: String,
    name: String
}

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

impl RSVP {
    pub fn new(person : Person, household_id: String) -> RSVP {
        RSVP {
            household_id,
            id: Uuid::new_v4().to_string(),
            name: person.name,
            email_address: person.email_address,
            attending: false.into(),
            invitation_submitted: false.into(),
            reminder_submitted: false.into()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Household {
    rsvps: Vec<RSVP>
}

impl Household {
    pub fn new(people: Vec<Person>) -> Household {
        let uuid = Uuid::new_v4().to_string();
        let mut rsvps : Vec<RSVP> = vec!();
        
        for person in people {
            rsvps.push(RSVP::new(person, uuid.clone()).clone());
        }

        Household {
            rsvps
        }
    }

    pub fn create_records(people: Vec<Person>) -> Result<Household, BatchWriteItemError> {
        let household = Household::new(people);
        let client = DynamoDbClient::new(Region::UsEast1);

        let put_requests : Vec<WriteRequest> = vec!(
            WriteRequest {
                put_request: Some(PutRequest {
                    item: serde_dynamodb::to_hashmap(&household.rsvps[0]).unwrap()
                }),
                ..WriteRequest::default()
            }
        );

        let mut request_items : HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert(env::var("RSVP_TABLE_NAME").unwrap(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items: request_items,
            ..BatchWriteItemInput::default()
        };

        match client.batch_write_item(batch_write_request_input).sync() {
            Ok(result) => {
                Ok(household)
            },
            Err(error) => {
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod rsvp_tests {

    use super::*;

    #[test]
    fn test_rsvp_new() {
        let household_id = Uuid::new_v4().to_string();
        let result = RSVP::new(
            Person {
                name: "Blaine Price".to_string(),
                email_address: "email@example.com".to_string()
            },
            household_id.clone()
        );

        assert_eq!(result.name, "Blaine Price".to_string());
        assert_eq!(result.email_address, "email@example.com".to_string());
        assert_eq!(result.household_id, household_id);
        assert_eq!(result.attending, false);
        assert_eq!(result.invitation_submitted, false);
        assert_eq!(result.reminder_submitted, false);
    }

    #[test]
    fn test_household_new() {
        let people : Vec<Person> = vec!(
            Person {
                email_address: "1example@email.com".to_string(),
                name: "person 1".to_string()
            },
            Person {
                email_address: "2example@email.com".to_string(),
                name: "person 2".to_string()
            }
        );

        let rsvps = Household::new(people).rsvps;
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_household_create_records() {
        let people : Vec<Person> = vec!(
            Person {
                email_address: "1example@email.com".to_string(),
                name: "person 1".to_string()
            },
            Person {
                email_address: "2example@email.com".to_string(),
                name: "person 2".to_string()
            }
        );

        let rsvps = Household::create_records(people).unwrap().rsvps;
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }
}