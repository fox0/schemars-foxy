#![allow(non_camel_case_types)]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information__spo_epgu_additional_information__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: (), /*TODO*/
    /// Хэш файла
    pub file_hash: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information__spo_epgu_additional_information {
    /// Уникальный идентификатор заявления
    pub id_application: (), /*TODO*/
    /// Комментарий
    pub comment: Option<() /*TODO*/>,
    pub file_list:
        Option<Vec<spo_epgu_additional_information__spo_epgu_additional_information__file_list>>,
}

pub type spo_epgu_additional_information__fui_type = String;
pub type spo_epgu_additional_information__file_hash_type = String;
pub type spo_epgu_additional_information__string1024 = String;
pub type spo_epgu_additional_information__int8 = u32;

/// tests/DespatchSchemas/spo_epgu_additional_information/spo_epgu_additional_information.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information {
    pub spo_epgu_additional_information:
        spo_epgu_additional_information__spo_epgu_additional_information,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__address_list__address {
    /// Полный адрес
    pub full_addr: (), /*TODO*/
    /// Идентификатор классификатора region_cls
    pub id_region: Option<() /*TODO*/>,
    /// Почтовый индекс
    pub index: Option<() /*TODO*/>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__address_list {
    pub address: spo_epgu_application__spo_epgu_application__entrant__address_list__address,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__legal_representative_info {
    /// ФИО представителя
    pub fio: (), /*TODO*/
    /// Телефон представителя
    pub phone: (), /*TODO*/
    /// Email представителя
    pub email: Option<() /*TODO*/>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__identification {
    /// Идентификатор документа
    pub id_identification: Option<() /*TODO*/>,
    /// Фамилия
    pub surname: Option<() /*TODO*/>,
    /// Имя
    pub name: Option<() /*TODO*/>,
    /// Отчество
    pub patronymic: Option<() /*TODO*/>,
    /// Серия документа
    pub series: (), /*TODO*/
    /// Номер документа
    pub number: (), /*TODO*/
    /// Дата выдачи документа
    pub issue_date: String,
    /// Кем выдан
    pub doc_org: (), /*TODO*/
    /// Код подразделения
    pub subdivision_code: Option<() /*TODO*/>,
    /// Тип документа
    pub id_document_type: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__photo {
    /// Фото поступающего
    pub file_hash: (), /*TODO*/
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant {
    /// Уникальный идентификатор абитуриента
    pub id_entrant: (), /*TODO*/
    /// Номер СНИЛС
    pub snils: Option<() /*TODO*/>,
    /// Идентификатор пола классификатор gender_cls
    pub id_gender: (), /*TODO*/
    /// Дата рождения
    pub birthday: String,
    /// Место рождения
    pub birthplace: (), /*TODO*/
    /// Номер телефона
    pub phone: Option<() /*TODO*/>,
    /// Электронная почта
    pub email: Option<() /*TODO*/>,
    /// Фамилия
    pub surname: (), /*TODO*/
    /// Имя
    pub name: (), /*TODO*/
    /// Отчество
    pub patronymic: Option<() /*TODO*/>,
    /// Идентификатор классификатор free_education_reason_cls
    pub id_free_education_reason: Option<() /*TODO*/>,
    pub address_list:
        Option<Vec<spo_epgu_application__spo_epgu_application__entrant__address_list>>,
    pub legal_representative_info:
        Option<spo_epgu_application__spo_epgu_application__entrant__legal_representative_info>,
    pub identification: spo_epgu_application__spo_epgu_application__entrant__identification,
    pub photo: Option<spo_epgu_application__spo_epgu_application__entrant__photo>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__application_specialty_list {
    /// Уникальный идентификатор специальности
    pub id_specialty: (), /*TODO*/
    /// Статус специальности (классификатор application_specialty_status_cls)
    pub id_application_specialty_status: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__benefit_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: (), /*TODO*/
    /// Хэш файла
    pub file_hash: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__benefit_list {
    /// Идентификатор классификатора benefit_cls
    pub id_benefit: (), /*TODO*/
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__benefit_list__file_list>,
    >,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__achievement_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: (), /*TODO*/
    /// Хэш файла
    pub file_hash: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__achievement_list {
    /// Идентификатор классификатора achievement_cls
    pub id_achievement: (), /*TODO*/
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__achievement_list__file_list>,
    >,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: (), /*TODO*/
    /// Хэш файла
    pub file_hash: (), /*TODO*/
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list__fields {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list {
    /// Уникальный идентификатор документа
    pub id_document: (), /*TODO*/
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__document_list__file_list>,
    >,
    /// Тип документа. Идентификатор классификатора document_type_cls
    pub id_document_type: (), /*TODO*/
    /// Серия документа
    pub doc_series: Option<() /*TODO*/>,
    /// Номер документа
    pub doc_number: Option<() /*TODO*/>,
    /// Организация, выдавшая документ
    pub doc_organization: Option<() /*TODO*/>,
    /// Дата выдачи. Шаблон '2006-01-02'
    pub issue_date: Option<String>,
    /// Статус документа. Идентификатор классификатора documents_check_status_cls
    pub id_check_status: Option<() /*TODO*/>,
    /// Время создания документа. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub created_date_time: String,
    /// Реквизиты согласно document_type_cls
    pub fields:
        Option<spo_epgu_application__spo_epgu_application__application__document_list__fields>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application {
    /// Уникальный идентификатор заявления
    pub id_application: (), /*TODO*/
    /// Дата регистрации заявления
    pub registration_date: String,
    /// Впервые в СПО
    pub first_time_spo: bool,
    /// Требуется общежитие
    pub need_hostel: bool,
    pub special_condition_list: Option<Vec<() /*TODO*/>>,
    /// Идентификатор статуса заявления (классификатор application_status_cls)
    pub id_application_status: (), /*TODO*/
    /// Список специальностей
    pub application_specialty_list:
        Vec<spo_epgu_application__spo_epgu_application__application__application_specialty_list>,
    pub benefit_list:
        Option<Vec<spo_epgu_application__spo_epgu_application__application__benefit_list>>,
    /// Список достижений
    pub achievement_list:
        Option<Vec<spo_epgu_application__spo_epgu_application__application__achievement_list>>,
    pub document_list:
        Option<Vec<spo_epgu_application__spo_epgu_application__application__document_list>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application {
    pub entrant: spo_epgu_application__spo_epgu_application__entrant,
    pub application: spo_epgu_application__spo_epgu_application__application,
}

pub type spo_epgu_application__int8 = u32;
pub type spo_epgu_application__snils_type = String;
pub type spo_epgu_application__int2 = u32;
pub type spo_epgu_application__int4 = u32;
pub type spo_epgu_application__string256 = String;
pub type spo_epgu_application__file_hash_type = String;
pub type spo_epgu_application__fui_type = String;
pub type spo_epgu_application__string6 = String;
pub type spo_epgu_application__string_25_type = String;
pub type spo_epgu_application__string50 = String;
pub type spo_epgu_application__string500 = String;
pub type spo_epgu_application__string1024 = String;

/// tests/DespatchSchemas/spo_epgu_application/spo_epgu_application.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application {
    pub spo_epgu_application: spo_epgu_application__spo_epgu_application,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application_cancel__spo_epgu_application_cancel {
    /// Уникальный идентификатор заявления
    pub id_application: (), /*TODO*/
    /// Дата отзыва заявления
    pub cancel_date: String,
    /// Причина
    pub reason: Option<() /*TODO*/>,
}

pub type spo_epgu_application_cancel__int8 = u32;
pub type spo_epgu_application_cancel__string1024 = String;

/// tests/DespatchSchemas/spo_epgu_application_cancel/spo_epgu_application_cancel.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application_cancel {
    pub spo_epgu_application_cancel: spo_epgu_application_cancel__spo_epgu_application_cancel,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_consent_to_enroll__spo_epgu_consent_to_enroll {
    /// Уникальный идентификатор заявления
    pub id_application: (), /*TODO*/
    /// Уникальный идентификатор специальности
    pub id_specialty: (), /*TODO*/
    /// Уникальный идентификатор поступающего
    pub id_entrant: (), /*TODO*/
    /// Согласие/отзыв на зачисление
    pub agree: bool,
    /// Дата подачи
    pub agree_date: String,
}

pub type spo_epgu_consent_to_enroll__int8 = u32;

/// tests/DespatchSchemas/spo_epgu_consent_to_enroll/spo_epgu_consent_to_enroll.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_consent_to_enroll {
    pub spo_epgu_consent_to_enroll: spo_epgu_consent_to_enroll__spo_epgu_consent_to_enroll,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_specialty_status__spo_sp_application_specialty_status {
    /// Уникальный идентификатор заявления (сущность spo_application_list)
    pub id_application: (), /*TODO*/
    /// Уникальный идентификатор специальности (сущность spo_secialty_list)
    pub id_specialty: (), /*TODO*/
    /// Статус cпециальности (классификатор application_specialty_status_cls)
    pub id_specialty_status: (), /*TODO*/
}

pub type spo_sp_application_specialty_status__int8 = u32;
pub type spo_sp_application_specialty_status__int4 = u32;

/// tests/DespatchSchemas/spo_sp_application_specialty_status/spo_sp_application_specialty_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_specialty_status {
    pub spo_sp_application_specialty_status:
        Option<spo_sp_application_specialty_status__spo_sp_application_specialty_status>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_status__spo_sp_application_status {
    /// Уникальный идентификатор заявления
    pub id_application: (), /*TODO*/
    /// Статус заявления (классификатор application_status_cls)
    pub id_application_status: (), /*TODO*/
}

pub type spo_sp_application_status__int8 = u32;
pub type spo_sp_application_status__int4 = u32;

/// tests/DespatchSchemas/spo_sp_application_status/spo_sp_application_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_status {
    pub spo_sp_application_status: spo_sp_application_status__spo_sp_application_status,
}
