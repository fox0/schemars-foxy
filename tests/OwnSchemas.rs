#![allow(non_camel_case_types)]

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct default_response__param_type {
    pub key: String,
    pub value: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct default_response__success_result_list {
    pub id_object: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct default_response__error_list {
    /// Уникальный код ошибки. Полный список кодов ошибок и их расшифровок смотрите в документации
    pub code: i32,
    /// Уникальный идентификатор объекта в рамках данного токена. Например, если в токене было передано три объекта, то каждому из них должен быть присвоен уникальный номер (IdObject), и тогда по каждому объекту вернется либо Success либо Error. При этом ответ по каждой отдельной сущности вернется только в случае успешно пройденной основной валидации
    pub id_object: Option<i32>,
    /// Описание кода ошибки
    pub description: String,
    /// Нестандартизированный параметр который раскрывает детали обработк
    pub params: Option<Vec<default_response__param_type>>,
    /// Время формирования ответа
    pub time: String,
}

/// tests/OwnSchemas/default.response.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct default_response {
    /// Уникальный идентификатор запроса на обработку данных (токена)
    pub id_jwt: i32,
    /// Имя сущности и действие над сущностью сконкатенированное через '_'. Например 'Campaign_Add'
    pub entity_action: String,
    /// Наименование стадии обработки на которой завершилось выполнение токена
    pub stage: String,
    pub success_result_list: Option<Vec<default_response__success_result_list>>,
    pub error_list: Option<Vec<default_response__error_list>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__id_jwt_by_entity {
    pub entity: String,
}

/// tests/OwnSchemas/id_jwt_by_entity/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub id_jwt_by_entity: get_direct__id_jwt_by_entity,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_addition_info_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления (spo_application_list)
    pub id_application: i32,
    /// Тип запроса
    pub id_additional_type: String,
}

/// tests/OwnSchemas/spo_addition_info_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_addition_info_list: Vec<add__spo_addition_info_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by__spo_addition_info_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления (spo_application_list)
    pub id_application: i32,
}

/// tests/OwnSchemas/spo_addition_info_list/get_by.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by {
    pub spo_addition_info_list: Vec<get_by__spo_addition_info_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_application_achievement_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Список достижений (если не передается - считается как отсутствие достижений)
    pub achievement_id_list: Option<Vec<i32>>,
}

/// tests/OwnSchemas/spo_application_achievement_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_application_achievement_list: Vec<edit__spo_application_achievement_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__spo_application_achievement_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
}

/// tests/OwnSchemas/spo_application_achievement_list/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub spo_application_achievement_list: Vec<get_direct__spo_application_achievement_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_application_benefit_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Список льгот (если не передается - считается как отсутствие льгот)
    pub benefit_id_list: Option<Vec<i32>>,
}

/// tests/OwnSchemas/spo_application_benefit_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_application_benefit_list: Vec<edit__spo_application_benefit_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__spo_application_benefit_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
}

/// tests/OwnSchemas/spo_application_benefit_list/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub spo_application_benefit_list: Vec<get_direct__spo_application_benefit_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_application_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id: i32,
    /// Признак необходимости общежития
    pub need_hostel: bool,
    /// Список специальных условий сдачи ВИ
    pub special_condition_id_list: Option<Vec<i32>>,
}

/// tests/OwnSchemas/spo_application_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_application_list: Vec<edit__spo_application_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__spo_application_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id: i32,
}

/// tests/OwnSchemas/spo_application_list/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub spo_application_list: Vec<get_direct__spo_application_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_application_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Уникальный идентификатор специальности (spo_specialty_list)
    pub id_specialty: i32,
}

/// tests/OwnSchemas/spo_application_speciality_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_application_specialty_list: Vec<add__spo_application_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by__spo_application_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления (spo_application_list)
    pub id_application: i32,
}

/// tests/OwnSchemas/spo_application_speciality_list/get_by.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by {
    pub spo_application_specialty_list: Vec<get_by__spo_application_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_application_specialty_status_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Уникальный идентификатор специальности (сущность spo_secialty_list)
    pub id_specialty: i32,
    /// Статус специальности (классификатор application_specialty_status_cls)
    pub id_application_specialty_status: i32,
    /// Комментарий к статусу
    pub status_comment: Option<String>,
}

/// tests/OwnSchemas/spo_application_specialty_status_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_application_specialty_status_list: Vec<edit__spo_application_specialty_status_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_application_status_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления
    pub id_application: i32,
    /// Статус заявления
    pub id_application_status: i32,
    /// Комментарий к статусу
    pub status_comment: Option<String>,
}

/// tests/OwnSchemas/spo_application_status_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_application_status_list: Option<Vec<edit__spo_application_status_list>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_campaign_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор приемной кампании
    pub uid: String,
    /// Наименование
    pub name: String,
    /// Учебный год приемной кампании в котором начинается прием
    pub year_start: i32,
    /// Часовой пояс относительно UTC (например, для Москвы = 3)
    pub org_time_zone: i32,
}

/// tests/OwnSchemas/spo_campaign_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_campaign_list: Vec<add__spo_campaign_list>,
}

/// Приемная кампания (сущность)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all__spo_campaign_list {
    pub id_object: i32,
}

/// tests/OwnSchemas/spo_campaign_list/get_all.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all {
    /// Приемная кампания (сущность)
    pub spo_campaign_list: Vec<get_all__spo_campaign_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by__spo_consent_to_enroll_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор поступающего
    pub id_entrant: i32,
}

/// tests/OwnSchemas/spo_consent_to_enroll_list/get_by.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by {
    pub spo_consent_to_enroll_list: Vec<get_by__spo_consent_to_enroll_list>,
}

/// Реквизиты согласно document_type_cls
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_document_list__fields {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_document_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор заявления
    pub id_application: i32,
    /// Серия
    pub doc_series: Option<String>,
    /// Номер
    pub doc_number: Option<String>,
    /// Дата выдачи
    pub issue_date: String,
    /// Организация, выдавшая документ
    pub doc_org: String,
    /// Тип документа
    pub id_document_type: i32,
    /// Реквизиты согласно document_type_cls
    pub fields: Option<add__spo_document_list__fields>,
}

/// tests/OwnSchemas/spo_document_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_document_list: Vec<add__spo_document_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_document_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор документа
    pub id: i32,
    /// Идентификатор статуса документа
    pub id_check_status: i32,
}

/// tests/OwnSchemas/spo_document_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_document_list: Vec<edit__spo_document_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove__spo_document_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор документа
    pub id: i32,
}

/// tests/OwnSchemas/spo_document_list/remove.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove {
    pub spo_document_list: Vec<remove__spo_document_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_entrance_test_schedule_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления (сущность spo_application_list)
    pub id_application: i32,
    /// Уникальный идентификатор специальности (сущность spo_specialty_list)
    pub id_specialty: i32,
    /// Название ВИ
    pub test_name: String,
    /// Место проведения ВИ
    pub test_place: String,
    pub test_date_time: String,
}

/// tests/OwnSchemas/spo_entrance_test_schedule_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_entrance_test_schedule_list: Vec<add__spo_entrance_test_schedule_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by__spo_entrance_test_schedule_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор заявления (сущность spo_application_list)
    pub id_application: i32,
    /// Уникальный идентификатор специальности (сущность spo_specialty_list)
    pub id_specialty: i32,
}

/// tests/OwnSchemas/spo_entrance_test_schedule_list/get_by.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by {
    pub spo_entrance_test_schedule_list: Vec<get_by__spo_entrance_test_schedule_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove__spo_document_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Идентификатор ВИ
    pub id: i32,
}

/// tests/OwnSchemas/spo_entrance_test_schedule_list/remove.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove {
    pub spo_document_list: Vec<remove__spo_document_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_entrant_identification_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор поступающего
    pub id_entrant: i32,
    /// Фамилия
    pub surname: String,
    /// Имя
    pub name: String,
    /// Отчество
    pub patronymic: Option<String>,
    /// Серия паспорта
    pub doc_series: String,
    /// Номер паспорта
    pub doc_number: String,
    /// Дата выдачи
    pub issue_date: String,
    /// Организация, выдавшая документ
    pub doc_org: String,
    /// Код подразделения
    pub subdivison_code: Option<String>,
    /// Тип документа
    pub id_document_type: i32,
    /// Установить как активный
    pub is_current: bool,
}

/// tests/OwnSchemas/spo_entrant_identification_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_entrant_identification_list: Vec<add__spo_entrant_identification_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_entrant_identification_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор ДУЛ, который надо сделать активным
    pub id: i32,
}

/// tests/OwnSchemas/spo_entrant_identification_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_entrant_identification_list: Vec<edit__spo_entrant_identification_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove__spo_entrant_identification_list {
    /// Идентификатор
    pub id_object: i32,
    /// Идентификатор ДУЛ
    pub id: i32,
}

/// tests/OwnSchemas/spo_entrant_identification_list/remove.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove {
    pub spo_entrant_identification_list: Vec<remove__spo_entrant_identification_list>,
}

/// Список адресов абитуриента
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_entrant_list__entrant__address_list {
    /// Полный адрес
    pub full_addr: String,
    /// Идентификатор классификатора region_cls
    pub id_region: Option<i32>,
    /// Индекс
    pub index: Option<String>,
    /// Признак адреса регистрации
    pub is_registration: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_entrant_list__entrant {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Id поступающего
    pub id: i32,
    /// СНИЛС - обязательный для граждан РФ
    pub snils: Option<String>,
    /// Идентификатор классификатора gender_cls
    pub id_gender: i32,
    /// Дата рождения. Шаблон "2006-01-02"
    pub birthday: String,
    /// Место рождения
    pub birthplace: String,
    /// Телефон
    pub phone: Option<String>,
    /// Электронный адрес
    pub email: Option<String>,
    /// Список адресов абитуриента
    pub address_list: Vec<edit__spo_entrant_list__entrant__address_list>,
}

/// Список профилей поступающих (сущность)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_entrant_list {
    pub entrant: Vec<edit__spo_entrant_list__entrant>,
}

/// tests/OwnSchemas/spo_entrant_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    /// Список профилей поступающих (сущность)
    pub spo_entrant_list: edit__spo_entrant_list,
}

/// Cписок профилей поступающих (сущность)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all__spo_entrant_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
}

/// tests/OwnSchemas/spo_entrant_list/get_all.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all {
    /// Cписок профилей поступающих (сущность)
    pub spo_entrant_list: Vec<get_all__spo_entrant_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__spo_entrant_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор поступающего
    pub id: i32,
}

/// tests/OwnSchemas/spo_entrant_list/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub spo_entrant_list: Vec<get_direct__spo_entrant_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_original_education_document_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор поступающего
    pub id_entrant: i32,
    /// Идентификатор заявления
    pub id_application: i32,
    /// Идентификатор специальности
    pub id_specialty: Option<i32>,
    /// Идентификатор документа об образовании, указаного в заявлении
    pub id_document: i32,
}

/// tests/OwnSchemas/spo_original_education_document_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_original_education_document_list: Vec<add__spo_original_education_document_list>,
}

/// Список документов об образовании, подлинники которых поданы поступающим
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by__spo_original_education_document_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор поступающего
    pub id_entrant: i32,
}

/// tests/OwnSchemas/spo_original_education_document_list/get_by.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_by {
    /// Список документов об образовании, подлинники которых поданы поступающим
    pub spo_original_education_document_list: Vec<get_by__spo_original_education_document_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove__spo_original_education_document_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор поступающего
    pub id_entrant: i32,
    /// Идентификатор заявления
    pub id_application: i32,
    /// Идентификатор специальности
    pub id_specialty: Option<i32>,
}

/// tests/OwnSchemas/spo_original_education_document_list/remove.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove {
    pub spo_original_education_document_list: Vec<remove__spo_original_education_document_list>,
}

/// Встроенный json объект соответствующий package_body_file.json (нет кодировки в base64)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__ranked_competition_list_package__embeded_package_body {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__ranked_competition_list_package {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Встроенный json объект соответствующий package_body_file.json (нет кодировки в base64)
    pub embeded_package_body: add__ranked_competition_list_package__embeded_package_body,
    /// Уникальный идентификатор специальности
    pub id_specialty: i32,
}

/// tests/OwnSchemas/spo_ranked_competition_list_package/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub ranked_competition_list_package: add__ranked_competition_list_package,
}

/// Пакет конкурсного списка (сущность)
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__ranked_competition_list_package {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Идентификатор конкурсного списка
    pub id: i32,
}

/// tests/OwnSchemas/spo_ranked_competition_list_package/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    /// Пакет конкурсного списка (сущность)
    pub ranked_competition_list_package: get_direct__ranked_competition_list_package,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__specialty_event_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Id специальности, сгенерированная Сервисом приема
    pub id_specialty: i32,
    /// Дата начала приема заявлений. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub reception_start: String,
    /// Дата окончания приема заявлений. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub reception_end: String,
    /// Дата окончания приема согласий. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub agreed_end: String,
}

/// tests/OwnSchemas/spo_specialty_event_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub specialty_event_list: Vec<edit__specialty_event_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add__spo_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор объекта в рамках организации
    pub uid: String,
    /// Идентификатор Приемной кампании
    pub id_campaign: i32,
    /// Идентификатор классификатора direction_cls
    pub id_direction: i32,
    /// Идентификатор классификатора education_level_cls
    pub id_education_level: i32,
    /// Идентификатор классификатора education_form_cls
    pub id_education_form: i32,
    /// Идентификатор классификатора payment_form_cls
    pub id_payment_form: i32,
    /// Количество мест
    pub number_places: i32,
    /// Необходимость расширенной медицинской справки
    pub medical_certificate: bool,
    /// Наличие приема на ЕПГУ
    pub online_application: bool,
    /// Наличие вступительных испытаний
    pub entrance_test_exists: bool,
    /// Срок обучения
    pub term_study: String,
    /// Дата начала приема заявлений. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub reception_start: String,
    /// Дата окончания приема заявлений. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub reception_end: String,
    /// Дата окончания приема согласий. Формат RFC3339 шаблон '2006-01-02T15:04:05+03:00'. Значение только по московскому времени
    pub agreed_end: String,
    /// Комментарий
    pub comment: Option<String>,
    pub subject_id_list: Option<Vec<i32>>,
}

/// tests/OwnSchemas/spo_specialty_list/add.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct add {
    pub spo_specialty_list: Vec<add__spo_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit__spo_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Id объекта (специальности)
    pub id: i32,
    /// Идентификатор классификатора education_level_cls
    pub id_education_level: i32,
    /// Идентификатор классификатора education_form_cls
    pub id_education_form: i32,
    /// Идентификатор классификатора payment_form_cls
    pub id_payment_form: i32,
    /// Количество мест
    pub number_places: i32,
    /// Нужна ли медицинская справка
    pub medical_certificate: bool,
    /// Наличие приема на ЕПГУ
    pub online_application: bool,
    /// Наличие вступительных испытаний
    pub entrance_test_exists: bool,
    /// Срок обучения
    pub term_study: String,
    /// Комментарий
    pub comment: Option<String>,
    pub subject_id_list: Option<Vec<i32>>,
}

/// tests/OwnSchemas/spo_specialty_list/edit.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct edit {
    pub spo_specialty_list: Vec<edit__spo_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all__spo_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Идентификатор Приемной кампании
    pub id_campaign: i32,
}

/// tests/OwnSchemas/spo_specialty_list/get_all.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_all {
    pub spo_specialty_list: Vec<get_all__spo_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct__spo_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Id объекта (специальности)
    pub id: i32,
}

/// tests/OwnSchemas/spo_specialty_list/get_direct.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct get_direct {
    pub spo_specialty_list: Vec<get_direct__spo_specialty_list>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove__spo_specialty_list {
    /// Уникальный идентификатор объекта в рамках данного токена
    pub id_object: i32,
    /// Уникальный идентификатор специальности
    pub id: i32,
}

/// tests/OwnSchemas/spo_specialty_list/remove.json
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct remove {
    pub spo_specialty_list: Vec<remove__spo_specialty_list>,
}
