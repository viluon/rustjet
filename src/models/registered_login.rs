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
pub struct RegisteredLogin {
    #[serde(rename = "accountCode")]
    pub account_code: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl RegisteredLogin {
    pub fn new(account_code: String, password: String) -> RegisteredLogin {
        RegisteredLogin {
            account_code,
            password,
        }
    }
}


