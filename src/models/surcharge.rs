/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */

/// Surcharge : Compulsory irreversible surcharge



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Surcharge {
    /// Surcharge amount (isnt included in basic line price or even in priceClass)
    #[serde(rename = "price")]
    pub price: f32,
    /// Notification which needs to be confirmed by customer
    #[serde(rename = "notification")]
    pub notification: String,
}

impl Surcharge {
    /// Compulsory irreversible surcharge
    pub fn new(price: f32, notification: String) -> Surcharge {
        Surcharge {
            price,
            notification,
        }
    }
}

