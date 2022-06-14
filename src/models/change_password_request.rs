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
pub struct ChangePasswordRequest {
    /// Old password
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    /// New password
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

impl ChangePasswordRequest {
    pub fn new(old_password: String, new_password: String) -> ChangePasswordRequest {
        ChangePasswordRequest {
            old_password,
            new_password,
        }
    }
}


