mod person;

pub use self::person::person::{PhoneNumber, PhoneType};
pub use person::*;

impl Person {
    pub fn new(
        name: impl Into<String>,
        id: i32,
        email: impl Into<String>,
        phones: impl Into<Vec<PhoneNumber>>,
    ) -> Self {
        Self {
            name: name.into(),
            id: id,
            email: email.into(),
            phones: phones.into(),
            ..Default::default()
        }
    }
}

impl PhoneNumber {
    pub fn new(number: impl Into<String>, phone_type: PhoneType) -> Self {
        Self {
            number: number.into(),
            phone_type: phone_type.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use prost::Message;
    use super::person::{person::*, Person};

    #[test]
    fn basics() {
        let phones = vec![PhoneNumber::new("111-222-3334", PhoneType::Mobile)];
        let person = Person::new("Tyr Chen", 1, "tyr@awesome.com", phones);
        let v1 = person.encode_to_vec();
        let person1 = Person::decode(v1.as_ref()).unwrap();
        assert_eq!(person, person1);

        let json = serde_json::to_string_pretty(&person1).unwrap();

        println!("{}", json);
    }
}
