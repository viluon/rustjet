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
pub struct Passenger {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// First name
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Surname
    #[serde(rename = "surname", skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Email are always required after first passenger if not pre-filled in user account (or customer isnt logged in)
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "tariff")]
    pub tariff: String,
    /// Basic price (ticket purchase) for tariff set on ticket with ticket currency (doesnt count with discounts)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    /// Final amount for passenger which will be refunded if canceled.
    #[serde(rename = "moneyBack", skip_serializing_if = "Option::is_none")]
    pub money_back: Option<f32>,
}

impl Passenger {
    pub fn new(tariff: String) -> Passenger {
        Passenger {
            id: None,
            first_name: None,
            surname: None,
            phone: None,
            email: None,
            date_of_birth: None,
            tariff,
            amount: None,
            money_back: None,
        }
    }
}


