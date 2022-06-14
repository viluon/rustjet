/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeUserRequest {
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "restrictPhoneNumbers", skip_serializing_if = "Option::is_none")]
    pub restrict_phone_numbers: Option<bool>,
    #[serde(rename = "companyInformation")]
    pub company_information: bool,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<crate::models::Company>>,
    #[serde(rename = "defaultTariffKey")]
    pub default_tariff_key: String,
    #[serde(rename = "notifications")]
    pub notifications: Box<crate::models::Notifications>,
}

impl ChangeUserRequest {
    pub fn new(company_information: bool, default_tariff_key: String, notifications: crate::models::Notifications) -> ChangeUserRequest {
        ChangeUserRequest {
            phone_number: None,
            restrict_phone_numbers: None,
            company_information,
            company: None,
            default_tariff_key,
            notifications: Box::new(notifications),
        }
    }
}


