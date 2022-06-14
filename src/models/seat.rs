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
pub struct Seat {
    /// Number of seat in vehicle
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "seatClass", skip_serializing_if = "Option::is_none")]
    pub seat_class: Option<String>,
    /// Notification which needs to be confirmed by customer before continue in reservation
    #[serde(rename = "seatConstraint", skip_serializing_if = "Option::is_none")]
    pub seat_constraint: Option<String>,
    /// Supplementary informations which are shown but isnt required to be confirmed
    #[serde(rename = "seatNotes", skip_serializing_if = "Option::is_none")]
    pub seat_notes: Option<Vec<String>>,
}

impl Seat {
    pub fn new() -> Seat {
        Seat {
            index: None,
            seat_class: None,
            seat_constraint: None,
            seat_notes: None,
        }
    }
}

