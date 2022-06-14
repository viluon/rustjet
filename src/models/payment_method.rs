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
pub struct PaymentMethod {
    /// Payment method code identifier
    #[serde(rename = "paymentMethodCode")]
    pub payment_method_code: String,
    #[serde(rename = "paymentType")]
    pub payment_type: crate::models::PaymentType,
    /// Checks if payment method is active
    #[serde(rename = "active")]
    pub active: bool,
    /// Payment method status (for example: why its not available)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Payment method availability expiration date-time
    #[serde(rename = "deadline", skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// Minutes left to end of payment method availability
    #[serde(rename = "deadlineAt", skip_serializing_if = "Option::is_none")]
    pub deadline_at: Option<i32>,
}

impl PaymentMethod {
    pub fn new(payment_method_code: String, payment_type: crate::models::PaymentType, active: bool) -> PaymentMethod {
        PaymentMethod {
            payment_method_code,
            payment_type,
            active,
            description: None,
            deadline: None,
            deadline_at: None,
        }
    }
}

