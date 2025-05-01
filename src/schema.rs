use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use indexmap::IndexMap;
use log::trace;
use serde::Deserialize;

/// JSON scheme
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "type")]
    pub instance_type: Option<InstanceType>,
    #[serde(rename = "$ref")]
    pub custom_type: Option<String>,
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
    pub definitions: Option<IndexMap<String, Schema>>,
    pub properties: Option<IndexMap<String, Schema>>,
    pub additional_properties: Option<IndexMap<String, String>>, // TODO
    pub unique_items: Option<bool>,
    pub items: Option<Box<Schema>>,
    pub minimum: Option<isize>,
    pub maximum: Option<isize>,
    pub format: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum InstanceType {
    Boolean,
    Object,
    Array,
    Integer,
    String,
}

impl Schema {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, serde_json::Error> {
        trace!("Schema::try_new({:?})", path.as_ref());

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn achievement_cls() {
        let _schema = Schema::try_new("tests/ClsSchemas/achievement_cls.json").unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn payment_form_cls() {
        let _schema = Schema::try_new("tests/ClsSchemas/payment_form_cls.json").unwrap();
        assert!(true);
    }

    #[test]
    fn spo_epgu_additional_information() {
        let _schema = Schema::try_new("tests/DespatchSchemas/spo_epgu_additional_information/spo_epgu_additional_information.json").unwrap();
        assert!(true);
    }

    #[test]
    fn spo_epgu_application() {
        let _schema =
            Schema::try_new("tests/DespatchSchemas/spo_epgu_application/spo_epgu_application.json")
                .unwrap();
        assert!(true);
    }
}
