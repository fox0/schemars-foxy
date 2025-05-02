use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use indexmap::IndexMap;
use log::trace;
use serde::Deserialize;

/// JSON scheme
#[derive(Debug)]
pub struct SchemaRoot {
    // pub path: Path,
    pub schema: Schema,
    pub definitions: IndexMap<String, Schema>,
}

#[derive(Debug)]
pub enum Schema {
    Object { properties: Vec<SchemaField> },
    Array { schema: Box<Schema> },
    Boolean,
    Integer,
    String,
    Custom(String),
}

#[derive(Debug)]
pub struct SchemaField {
    pub description: Option<String>,
    pub name: String,
    pub is_required: bool,
    pub schema: Schema,
}

impl SchemaRoot {
    pub fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, serde_json::Error> {
        trace!("SchemaRoot::try_new({:?})", path.as_ref());
        let schema = SchemaDeserialize::try_new(path)?;
        assert_eq!(schema.instance_type, Some(InstanceType::Object));

        let mut definitions = IndexMap::new();
        if let Some(defs) = schema.definitions.clone() {
            for (name, schema2) in defs {
                definitions.insert(name, Schema::from(schema2));
            }
        }

        let schema = Schema::from(schema);
        Ok(Self {
            definitions,
            schema,
        })
    }
}

impl From<SchemaDeserialize> for Schema {
    fn from(schema: SchemaDeserialize) -> Self {
        trace!("Schema::from<SchemaDeserialize>");

        match schema.instance_type {
            Some(InstanceType::Object) => {
                assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                // assert!(schema.required.is_none());
                // assert!(schema.definitions.is_none());
                // assert!(schema.properties.is_none());
                // assert!(schema.additional_properties.is_none());  // ignore
                assert!(schema.unique_items.is_none());
                assert!(schema.items.is_none());
                assert!(schema.minimum.is_none());
                assert!(schema.maximum.is_none());
                assert!(schema.min_length.is_none());
                assert!(schema.max_length.is_none());
                assert!(schema.format.is_none());
                assert!(schema.pattern.is_none());
                assert!(schema.enum_values.is_none());

                let required = schema.required.unwrap_or_default();

                let mut properties = vec![];
                if let Some(pros) = schema.properties {
                    for (name, schema2) in pros {
                        properties.push(SchemaField {
                            description: schema2.description.clone(),
                            name: name.clone(),
                            is_required: required.contains(&name),
                            schema: Self::from(schema2),
                        });
                    }
                }

                Self::Object { properties }
            }
            Some(InstanceType::Array) => {
                assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                assert!(schema.required.is_none());
                assert!(schema.definitions.is_none());
                assert!(schema.properties.is_none());
                assert!(schema.additional_properties.is_none());
                // assert!(schema.unique_items.is_none());  // ignore
                // assert!(schema.items.is_none());
                assert!(schema.minimum.is_none());
                assert!(schema.maximum.is_none());
                assert!(schema.min_length.is_none());
                assert!(schema.max_length.is_none());
                assert!(schema.format.is_none());
                assert!(schema.pattern.is_none());
                assert!(schema.enum_values.is_none());

                let schema2 = *schema.items.unwrap();
                Self::Array {
                    schema: Box::new(Self::from(schema2)),
                }
            }
            Some(InstanceType::Boolean) => {
                assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                assert!(schema.required.is_none());
                assert!(schema.definitions.is_none());
                assert!(schema.properties.is_none());
                assert!(schema.additional_properties.is_none());
                assert!(schema.unique_items.is_none());
                assert!(schema.items.is_none());
                assert!(schema.minimum.is_none());
                assert!(schema.maximum.is_none());
                assert!(schema.min_length.is_none());
                assert!(schema.max_length.is_none());
                assert!(schema.format.is_none());
                assert!(schema.pattern.is_none());
                assert!(schema.enum_values.is_none());

                Self::Boolean
            }
            Some(InstanceType::Integer) => {
                assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                assert!(schema.required.is_none());
                assert!(schema.definitions.is_none());
                assert!(schema.properties.is_none());
                assert!(schema.additional_properties.is_none());
                assert!(schema.unique_items.is_none());
                assert!(schema.items.is_none());
                // assert!(schema.minimum.is_none());  // ignore
                // assert!(schema.maximum.is_none());  // ignore
                assert!(schema.min_length.is_none());
                assert!(schema.max_length.is_none());
                assert!(schema.format.is_none());
                assert!(schema.pattern.is_none());
                assert!(schema.enum_values.is_none());

                // /// return: type_name
                // #[must_use]
                // fn parse_number(&mut self, schema: Schema) -> String {
                //     trace!("Walker::parse_number()");
                //     debug_assert_eq!(schema.instance_type, Some(InstanceType::Integer));

                //     if schema.minimum.unwrap_or(-1) >= 0 {
                //         "u32".into()
                //     } else {
                //         "i32".into()
                //     }
                // }

                Self::Integer
            }
            Some(InstanceType::String) => {
                assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                assert!(schema.required.is_none());
                assert!(schema.definitions.is_none());
                assert!(schema.properties.is_none());
                assert!(schema.additional_properties.is_none());
                assert!(schema.unique_items.is_none());
                assert!(schema.items.is_none());
                assert!(schema.minimum.is_none());
                assert!(schema.maximum.is_none());
                // assert!(schema.min_length.is_none());  // ignore
                // assert!(schema.max_length.is_none());  // ignore
                // assert!(schema.format.is_none());  // ignore
                // assert!(schema.pattern.is_none());  // ignore
                // assert!(schema.enum_values.is_none());  // ignore

                Self::String
            }
            None => {
                // assert!(schema.custom_type.is_none());
                // assert!(schema.description.is_none());
                assert!(schema.required.is_none());
                assert!(schema.definitions.is_none());
                assert!(schema.properties.is_none());
                assert!(schema.additional_properties.is_none());
                assert!(schema.unique_items.is_none());
                assert!(schema.items.is_none());
                assert!(schema.minimum.is_none());
                assert!(schema.maximum.is_none());
                assert!(schema.min_length.is_none());
                assert!(schema.max_length.is_none());
                assert!(schema.format.is_none());
                assert!(schema.pattern.is_none());
                assert!(schema.enum_values.is_none());

                let custom_type = schema
                    .custom_type
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap()
                    .to_string();
                Self::Custom(custom_type)
            }
        }
    }
}

/// JSON scheme
#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
struct SchemaDeserialize {
    #[serde(rename = "type")]
    instance_type: Option<InstanceType>,
    // #[serde(rename = "type")]
    // instance_type: Option<Vec<InstanceType>>,
    #[serde(rename = "$ref")]
    custom_type: Option<String>,
    description: Option<String>,
    required: Option<HashSet<String>>,
    definitions: Option<IndexMap<String, SchemaDeserialize>>,
    properties: Option<IndexMap<String, SchemaDeserialize>>,
    additional_properties: Option<IndexMap<String, String>>, // TODO
    unique_items: Option<bool>,
    min_items: Option<i32>,
    max_items: Option<i32>,
    items: Option<Box<SchemaDeserialize>>,
    minimum: Option<isize>,
    maximum: Option<isize>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    format: Option<String>,
    pattern: Option<String>,
    #[serde(rename = "enum")]
    enum_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
enum InstanceType {
    Object,
    Array,
    Boolean,
    Integer,
    String,
}

impl SchemaDeserialize {
    fn try_new<P: AsRef<Path>>(path: P) -> Result<Self, serde_json::Error> {
        trace!("SchemaDeserialize::try_new({:?})", path.as_ref());

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn init() {
        colog::default_builder()
            .filter(None, log::LevelFilter::Trace)
            .default_format()
            .format_source_path(true)
            .init();
    }

    #[test]
    fn achievement_cls() {
        let _schema = SchemaDeserialize::try_new("tests/ClsSchemas/achievement_cls.json").unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn payment_form_cls() {
        let _schema = SchemaDeserialize::try_new("tests/ClsSchemas/payment_form_cls.json").unwrap();
        assert!(true);
    }

    #[test]
    fn spo_epgu_additional_information() {
        let _schema = SchemaDeserialize::try_new("tests/DespatchSchemas/spo_epgu_additional_information/spo_epgu_additional_information.json").unwrap();
        assert!(true);
    }

    #[test]
    fn spo_epgu_application() {
        let _schema = SchemaDeserialize::try_new(
            "tests/DespatchSchemas/spo_epgu_application/spo_epgu_application.json",
        )
        .unwrap();
        assert!(true);
    }

    #[test]
    fn default_response() {
        let _schema = SchemaDeserialize::try_new("tests/OwnSchemas/default.response.json").unwrap();
        assert!(true);
    }

    #[ignore]
    #[test]
    fn id_jwt_by_entity_get_direct_response() {
        let _schema = SchemaDeserialize::try_new(
            "tests/OwnSchemas/id_jwt_by_entity/get_direct.response.json",
        )
        .unwrap();
        dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn spo_addition_info_list_add() {
        let _schema =
            SchemaDeserialize::try_new("tests/OwnSchemas/spo_addition_info_list/add.json").unwrap();
        assert!(true);
    }

    #[test]
    fn achievement_cls2() {
        // init();
        let _schema = SchemaRoot::try_new("tests/ClsSchemas/achievement_cls.json").unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn payment_form_cls2() {
        let _schema = SchemaRoot::try_new("tests/ClsSchemas/payment_form_cls.json").unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn spo_epgu_additional_information2() {
        let _schema = SchemaRoot::try_new("tests/DespatchSchemas/spo_epgu_additional_information/spo_epgu_additional_information.json").unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn spo_epgu_application2() {
        let _schema = SchemaRoot::try_new(
            "tests/DespatchSchemas/spo_epgu_application/spo_epgu_application.json",
        )
        .unwrap();
        // dbg!(_schema);
        assert!(true);
    }

    #[test]
    fn default_response2() {
        let _schema = SchemaRoot::try_new("tests/OwnSchemas/default.response.json").unwrap();
        assert!(true);
    }
}
