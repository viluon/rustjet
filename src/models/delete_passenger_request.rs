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
pub struct DeletePassengerRequest {
    /// List of seats which is about to be canceled. Reservation in non-seat-reservation connection isnt included in this list.
    #[serde(rename = "seats")]
    pub seats: Vec<crate::models::SelectedSeat>,
    #[serde(rename = "refundToOriginalSource")]
    pub refund_to_original_source: bool,
}

impl DeletePassengerRequest {
    pub fn new(seats: Vec<crate::models::SelectedSeat>, refund_to_original_source: bool) -> DeletePassengerRequest {
        DeletePassengerRequest {
            seats,
            refund_to_original_source,
        }
    }
}


