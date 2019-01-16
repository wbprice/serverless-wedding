use serde_derive::{Serialize, Deserialize};
use std::vec::{Vec};
use std::collections::{HashMap};
use std::env;
use uuid::Uuid;
use log::{info, error};

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, AttributeValue, QueryInput, QueryError, PutRequest, PutItemError, DynamoDbClient, WriteRequest, BatchWriteItemInput, BatchWriteItemError};
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

    pub fn batch_new(people: Vec<Person>) -> Vec<RSVP> {
        let uuid = Uuid::new_v4().to_string();
        let mut rsvps : Vec<RSVP> = vec!();
        
        for person in people {
            rsvps.push(RSVP::new(person, uuid.clone()).clone());
        }

        rsvps
    }

    pub fn list_by_household_id(uuid: Uuid) -> Result<Vec<RSVP>, QueryError> {
        let client = DynamoDbClient::new(Region::UsEast1);

        let mut query = HashMap::new();
        query.insert(String::from(":household_id"), AttributeValue {
            s: Some(uuid.to_string()),
            ..Default::default()
        });

        info!("Created the query");

        let query_input = QueryInput {
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        };

        info!("Created QueryInput");

        let rsvps = client.query(query_input)
            .sync()
            .unwrap_or_else(|_| {
                error!("Issue performing the query");
                panic!("Issue performing the query");
            })
            .items
            .unwrap_or_else(|| {
                error!("Issue unwrapping the response");
                vec![]
            })
            .into_iter()
            .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
            .collect();

        info!("Retrieved the results!");

        Ok(rsvps)
    }
    
    pub fn batch_create_records(people: Vec<Person>) -> Result<Vec<RSVP>, BatchWriteItemError> {
        let rsvps = RSVP::batch_new(people); 
        let client = DynamoDbClient::new(Region::UsEast1);

        let mut put_requests : Vec<WriteRequest> = vec!();
        for rsvp in &rsvps {
            put_requests.push(
                WriteRequest {
                    put_request: Some(PutRequest {
                        item: serde_dynamodb::to_hashmap(&rsvp).unwrap()
                    }),
                    ..WriteRequest::default()
                }
            )
        }

        let mut request_items : HashMap<String, Vec<WriteRequest>> = HashMap::new();
        request_items.insert(env::var("RSVP_TABLE_NAME").unwrap(), put_requests);

        let batch_write_request_input = BatchWriteItemInput {
            request_items: request_items,
            ..BatchWriteItemInput::default()
        };

        match client.batch_write_item(batch_write_request_input).sync() {
            Ok(_result) => {
                Ok(rsvps)
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
    fn test_rsvp_batch_new() {
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

        let rsvps = RSVP::batch_new(people);
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_rsvp_batch_create_records() {
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

        let rsvps = RSVP::batch_create_records(people).unwrap();
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_rsvp_list_by_househould_id() {
        let uuid = Uuid::parse_str("3eb28445-7698-4a00-b071-49da8eaac944").unwrap();
        let rsvps = RSVP::list_by_household_id(uuid).unwrap();
        assert_eq!(rsvps.len(), 2);
    }
}