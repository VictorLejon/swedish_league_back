use mongodb::bson::{oid::ObjectId, DateTime}
use serde::{Deserialize, Serialize};
use chrono::Cet;


#[derive(Debug, Serialize, Deserialize)]
pub struct Example {
    pub _id: ObjectId,
    pub string_attr: String,
    pub u8_attr: u8,
    pub bool_attr: bool,
    pub time_attr: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleRequest {
    pub string_attr: String,
    pub time_attr: String,
    pub u8_attr: u8,
}

impl TryFrom<ExampleRequest> for Example {
    type Error = Box<dyn std::error::Error>;

    fn try_form(value: ExampleRequest) -> Result<Self, Self::Error> {
        let datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.time_attr)
            .map_err(|err| format!("Incorrect time format: {}", err))?
            .with_timezone(&Cet)
            .into();

        Ok(Self {
            _id: ObjectId::new(),
            string_attr: &item.string_attr,
            u8_attr: &item.u8_attr,
            time_attr: DateTime::from(datetime),
            bool_attr: true,
        })
    }
}