#![allow(non_camel_case_types)]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct achievement_cls__achievement_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/achievement_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct achievement_cls {
    pub achievement_cls: Option<Vec<achievement_cls__achievement_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct application_specialty_status_cls__application_specialty_status_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/application_specialty_status_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct application_specialty_status_cls {
    pub application_specialty_status_cls:
        Option<Vec<application_specialty_status_cls__application_specialty_status_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct application_status_cls__application_status_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/application_status_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct application_status_cls {
    pub application_status_cls: Option<Vec<application_status_cls__application_status_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct benefit_cls__benefit_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/benefit_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct benefit_cls {
    pub benefit_cls: Option<Vec<benefit_cls__benefit_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct direction_cls__direction_cls {
    /// Идентификатор
    pub id: i32,
    /// Уникальный код
    pub code: String,
    /// Наименование
    pub name: String,
    /// Идентификатор верхнего уровня
    pub id_parent: Option<i32>,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/direction_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct direction_cls {
    pub direction_cls: Option<Vec<direction_cls__direction_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct document_type_cls__document_type_cls__fields_description {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct document_type_cls__document_type_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Реквизиты из расширенного списка в формате json (см. пункт 'Документы поступающего. Общее описание' Инструкции API)
    pub fields_description: Option<document_type_cls__document_type_cls__fields_description>,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/document_type_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct document_type_cls {
    pub document_type_cls: Option<Vec<document_type_cls__document_type_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct documents_check_status__documents_check_status {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/documents_check_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct documents_check_status {
    pub documents_check_status: Option<Vec<documents_check_status__documents_check_status>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct education_form_cls__education_form_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/education_form_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct education_form_cls {
    pub education_form_cls: Option<Vec<education_form_cls__education_form_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct education_level_cls__education_level_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/education_level_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct education_level_cls {
    pub education_level_cls: Option<Vec<education_level_cls__education_level_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct free_education_reasons_cls__free_education_reasons_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/free_education_reasons_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct free_education_reasons_cls {
    pub free_education_reasons_cls:
        Option<Vec<free_education_reasons_cls__free_education_reasons_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct gender_cls__gender_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/gender_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct gender_cls {
    pub gender_cls: Option<Vec<gender_cls__gender_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct oksm_cls__oksm_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/oksm_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct oksm_cls {
    pub oksm_cls: Option<Vec<oksm_cls__oksm_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct packages_status_cls__packages_status_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/packages_status_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct packages_status_cls {
    pub packages_status_cls: Option<Vec<packages_status_cls__packages_status_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct payment_form_cls__payment_form_cls {
    /// Идентификатор
    pub id: i32,
    pub code_esnsi: Option<String>,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/payment_form_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct payment_form_cls {
    pub payment_form_cls: Option<Vec<payment_form_cls__payment_form_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct regions_cls__regions_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/regions_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct regions_cls {
    pub regions_cls: Option<Vec<regions_cls__regions_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct special_conditions_cls__special_conditions_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
}

/// tests/ClsSchemas/special_conditions_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct special_conditions_cls {
    pub special_conditions_cls: Option<Vec<special_conditions_cls__special_conditions_cls>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct subjects_cls__subjects_cls {
    /// Идентификатор
    pub id: i32,
    /// Наименование
    pub name: String,
    /// Признак актуальности
    pub actual: bool,
    /// Время создания
    pub created_at: String,
    pub code_esnsi: Option<String>,
}

/// tests/ClsSchemas/subjects_cls.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct subjects_cls {
    pub subjects_cls: Option<Vec<subjects_cls__subjects_cls>>,
}
