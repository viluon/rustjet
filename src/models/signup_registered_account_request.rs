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
pub struct SignupRegisteredAccountRequest {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "surname")]
    pub surname: String,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Restrict phone number for work with sms
    #[serde(rename = "restrictPhoneNumbers", skip_serializing_if = "Option::is_none")]
    pub restrict_phone_numbers: Option<bool>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "mojeid", skip_serializing_if = "Option::is_none")]
    pub mojeid: Option<String>,
    #[serde(rename = "companyInformation", skip_serializing_if = "Option::is_none")]
    pub company_information: Option<bool>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<Box<crate::models::Company>>,
    #[serde(rename = "defaultTariff")]
    pub default_tariff: String,
    #[serde(rename = "currency")]
    pub currency: crate::models::Currency,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "notifications")]
    pub notifications: Box<crate::models::Notifications>,
    /// Agree with terms
    #[serde(rename = "agreeWithTerms")]
    pub agree_with_terms: bool,
}

impl SignupRegisteredAccountRequest {
    pub fn new(first_name: String, surname: String, email: String, default_tariff: String, currency: crate::models::Currency, password: String, notifications: crate::models::Notifications, agree_with_terms: bool) -> SignupRegisteredAccountRequest {
        SignupRegisteredAccountRequest {
            first_name,
            surname,
            phone_number: None,
            restrict_phone_numbers: None,
            email,
            mojeid: None,
            company_information: None,
            company: None,
            default_tariff,
            currency,
            password,
            notifications: Box::new(notifications),
            agree_with_terms,
        }
    }
}

