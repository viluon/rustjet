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
pub struct LayoutFooter {
    #[serde(rename = "paymentMethods", skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<crate::models::LayoutFooterPaymentMethods>>,
    #[serde(rename = "formURL", skip_serializing_if = "Option::is_none")]
    pub form_url: Option<Box<crate::models::LayoutFooterFormUrl>>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl LayoutFooter {
    pub fn new() -> LayoutFooter {
        LayoutFooter {
            payment_methods: None,
            form_url: None,
            phone: None,
            email: None,
        }
    }
}

