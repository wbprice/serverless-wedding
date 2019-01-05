use serde_derive::{{Serialize, Deserialize}};
use std::vec::{{Vec}};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    email_address: String,
    name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct People {
    people: Vec<Person>
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
    pub fn new(person : Person, uuid: Uuid) -> RSVP {
        RSVP {
            household_id: uuid::to_string(),
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
        let mut rsvps : Vec<RSVP> = Vec::new();
        
        for person in people {
            rsvps.push(RSVP::new(person).clone());
        }

        Household {
            rsvps
        }
    }
}

#[cfg(test)]
mod rsvp_tests {

    use super::*;

    // #[test]
    // // fn test_rsvp_new() {
    // //     let result = RSVP::new(Person {
    // //         name: "Blaine Price".to_string(), 
    // //         email_address: "email@example.com".to_string()
    // //     });

    // //     assert_eq!(result.name, "Blaine Price".to_string());
    // //     assert_eq!(result.email_address, "email@example.com".to_string());
    // //     assert_eq!(result.attending, false);
    // //     assert_eq!(result.invitation_submitted, false);
    // //     assert_eq!(result.reminder_submitted, false);
    // // }

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

        let household = Household::new(people);
        println!("{:?}", household);
    }
}