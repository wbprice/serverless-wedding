use serde_derive::{Serialize, Deserialize};
use std::vec::{Vec};
use std::collections::{HashMap};
use std::env;
use uuid::Uuid;
use log::{debug, info, error};
use std::error::Error;

use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb,
    AttributeValue,
    QueryInput,
    QueryError,
    PutRequest,
    DynamoDbClient,
    WriteRequest,
    BatchWriteItemInput,
    BatchWriteItemError,
    UpdateItemInput,
    UpdateItemError
};
use serde_dynamodb;

use crate::models::{RSVP, Person};

pub struct Household;

impl Household {
    pub fn new(people: Vec<Person>) -> Vec<RSVP> {
        let uuid = Uuid::new_v4().to_string();
        let mut rsvps : Vec<RSVP> = vec!();
        
        for person in people {
            rsvps.push(RSVP::new(person, uuid.clone()).clone());
        }

        rsvps
    }

    pub fn create(people: Vec<Person>) -> Result<Vec<RSVP>, BatchWriteItemError> {
        let rsvps = Household::new(people); 
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

    pub fn get(uuid: Uuid) -> Result<Vec<RSVP>, Box<Error>> {
        let client = DynamoDbClient::new(Region::UsEast1);

        let mut query = HashMap::new();
        query.insert(String::from(":household_id"), AttributeValue {
            s: Some(uuid.to_string()),
            ..Default::default()
        });

        let query_input = QueryInput {
            table_name: env::var("RSVP_TABLE_NAME").unwrap(),
            key_condition_expression: Some("household_id = :household_id".to_string()),
            expression_attribute_values: Some(query),
            ..QueryInput::default()
        };

        match client.query(query_input).sync() {
            Ok(response) => {
                match response.items {
                    Some(items) => {
                        let rsvps = items.into_iter()
                            .map(|item| serde_dynamodb::from_hashmap(item).unwrap())
                            .collect();
                        Ok(rsvps)
                    },
                    None => {
                        error!("No results!");
                        Ok(vec![])
                    }
                }
            },
            Err(error) => {
                error!("There was an error performing the query {}", error);
                Ok(vec![])
            }
        }
    }
}


#[cfg(test)]
mod household_tests {

    use super::*;
    
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

        let rsvps = Household::new(people);
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

        let rsvps = RSVP::batch_create_records(people).unwrap();
        assert_eq!(rsvps[0].household_id, rsvps[1].household_id);
    }

    #[test]
    fn test_household_get() {
        let uuid = Uuid::parse_str("3eb28445-7698-4a00-b071-49da8eaac944").unwrap();
        let rsvps = RSVP::list_by_household_id(uuid).unwrap();
        assert_eq!(rsvps.len(), 2);
    }
}
