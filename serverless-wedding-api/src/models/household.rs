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