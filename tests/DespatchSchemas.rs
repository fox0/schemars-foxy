#![allow(non_camel_case_types)]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information__spo_epgu_additional_information__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: String,
    /// Хэш файла
    pub file_hash: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_additional_information__spo_epgu_additional_information {
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Комментарий
    pub comment: Option<String>,
    pub file_list:
        Option<Vec<spo_epgu_additional_information__spo_epgu_additional_information__file_list>>,
}

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
    pub full_addr: String,
    /// Идентификатор классификатора region_cls
    pub id_region: Option<i32>,
    /// Почтовый индекс
    pub index: Option<String>,
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
    pub fio: String,
    /// Телефон представителя
    pub phone: String,
    /// Email представителя
    pub email: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__identification {
    /// Идентификатор документа
    pub id_identification: Option<i32>,
    /// Фамилия
    pub surname: Option<String>,
    /// Имя
    pub name: Option<String>,
    /// Отчество
    pub patronymic: Option<String>,
    /// Серия документа
    pub series: String,
    /// Номер документа
    pub number: String,
    /// Дата выдачи документа
    pub issue_date: String,
    /// Кем выдан
    pub doc_org: String,
    /// Код подразделения
    pub subdivision_code: Option<String>,
    /// Тип документа
    pub id_document_type: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant__photo {
    /// Фото поступающего
    pub file_hash: String,
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__entrant {
    /// Уникальный идентификатор абитуриента
    pub id_entrant: i32,
    /// Номер СНИЛС
    pub snils: Option<String>,
    /// Идентификатор пола классификатор gender_cls
    pub id_gender: i32,
    /// Дата рождения
    pub birthday: String,
    /// Место рождения
    pub birthplace: String,
    /// Номер телефона
    pub phone: Option<String>,
    /// Электронная почта
    pub email: Option<String>,
    /// Фамилия
    pub surname: String,
    /// Имя
    pub name: String,
    /// Отчество
    pub patronymic: Option<String>,
    /// Идентификатор классификатор free_education_reason_cls
    pub id_free_education_reason: Option<i32>,
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
    pub id_specialty: i32,
    /// Статус специальности (классификатор application_specialty_status_cls)
    pub id_application_specialty_status: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__benefit_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: String,
    /// Хэш файла
    pub file_hash: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__benefit_list {
    /// Идентификатор классификатора benefit_cls
    pub id_benefit: i32,
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__benefit_list__file_list>,
    >,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__achievement_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: String,
    /// Хэш файла
    pub file_hash: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__achievement_list {
    /// Идентификатор классификатора achievement_cls
    pub id_achievement: i32,
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__achievement_list__file_list>,
    >,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list__file_list {
    /// Уникальный идентификатор файла в Сервисе Приема. По этому идентификатору можно получить файл в /api/file/get
    pub fui: String,
    /// Хэш файла
    pub file_hash: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list__fields {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_epgu_application__spo_epgu_application__application__document_list {
    /// Уникальный идентификатор документа
    pub id_document: i32,
    pub file_list: Option<
        Vec<spo_epgu_application__spo_epgu_application__application__document_list__file_list>,
    >,
    /// Тип документа. Идентификатор классификатора document_type_cls
    pub id_document_type: i32,
    /// Серия документа
    pub doc_series: Option<String>,
    /// Номер документа
    pub doc_number: Option<String>,
    /// Организация, выдавшая документ
    pub doc_organization: Option<String>,
    /// Дата выдачи. Шаблон '2006-01-02'
    pub issue_date: Option<String>,
    /// Статус документа. Идентификатор классификатора documents_check_status_cls
    pub id_check_status: Option<i32>,
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
    pub id_application: i32,
    /// Дата регистрации заявления
    pub registration_date: String,
    /// Впервые в СПО
    pub first_time_spo: bool,
    /// Требуется общежитие
    pub need_hostel: bool,
    pub special_condition_list: Option<Vec<i32>>,
    /// Идентификатор статуса заявления (классификатор application_status_cls)
    pub id_application_status: i32,
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
    pub id_application: i32,
    /// Дата отзыва заявления
    pub cancel_date: String,
    /// Причина
    pub reason: Option<String>,
}

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
    pub id_application: i32,
    /// Уникальный идентификатор специальности
    pub id_specialty: i32,
    /// Уникальный идентификатор поступающего
    pub id_entrant: i32,
    /// Согласие/отзыв на зачисление
    pub agree: bool,
    /// Дата подачи
    pub agree_date: String,
}

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
    pub id_application: i32,
    /// Уникальный идентификатор специальности (сущность spo_secialty_list)
    pub id_specialty: i32,
    /// Статус cпециальности (классификатор application_specialty_status_cls)
    pub id_specialty_status: i32,
}

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
    pub id_application: i32,
    /// Статус заявления (классификатор application_status_cls)
    pub id_application_status: i32,
}

/// tests/DespatchSchemas/spo_sp_application_status/spo_sp_application_status.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct spo_sp_application_status {
    pub spo_sp_application_status: spo_sp_application_status__spo_sp_application_status,
}
