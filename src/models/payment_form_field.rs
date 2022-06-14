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
pub struct PaymentFormField {
    /// Label name
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[serde(rename = "fieldType")]
    pub field_type: crate::models::PaymentFormFieldType,
}

impl PaymentFormField {
    pub fn new(field_name: String, field_type: crate::models::PaymentFormFieldType) -> PaymentFormField {
        PaymentFormField {
            field_name,
            field_type,
        }
    }
}


