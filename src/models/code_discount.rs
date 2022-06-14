/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */

/// CodeDiscount : Flat rate discount



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeDiscount {
    /// Flat rate discount ID
    #[serde(rename = "id")]
    pub id: i64,
    /// Five-digit discount code
    #[serde(rename = "code")]
    pub code: String,
    /// Flat rate discount sum in ticket currency
    #[serde(rename = "discount")]
    pub discount: f32,
}

impl CodeDiscount {
    /// Flat rate discount
    pub fn new(id: i64, code: String, discount: f32) -> CodeDiscount {
        CodeDiscount {
            id,
            code,
            discount,
        }
    }
}

