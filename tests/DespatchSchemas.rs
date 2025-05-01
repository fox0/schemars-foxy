#![allow(non_camel_case_types)]
#![rustfmt::skip]

/// tests/DespatchSchemas/spo_epgu_additional_information/spo_epgu_additional_information.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information {
    pub spo_epgu_additional_information: (),
}

/// tests/DespatchSchemas/spo_epgu_application/spo_epgu_application.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application {
    pub spo_epgu_application: (),
}

/// tests/DespatchSchemas/spo_epgu_application_cancel/spo_epgu_application_cancel.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application_cancel {
    pub spo_epgu_application_cancel: (),
}

/// tests/DespatchSchemas/spo_epgu_consent_to_enroll/spo_epgu_consent_to_enroll.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_consent_to_enroll {
    pub spo_epgu_consent_to_enroll: (),
}

/// tests/DespatchSchemas/spo_sp_application_specialty_status/spo_sp_application_specialty_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_specialty_status {
    pub spo_sp_application_specialty_status: Option<()>,
}

/// tests/DespatchSchemas/spo_sp_application_status/spo_sp_application_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_status {
    pub spo_sp_application_status: (),
}

