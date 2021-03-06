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
pub struct RouteSeatsRequest {
    #[serde(rename = "sections")]
    pub sections: Vec<crate::models::SimpleSection>,
    #[serde(rename = "tariffs")]
    pub tariffs: Vec<String>,
    #[serde(rename = "seatClass")]
    pub seat_class: String,
}

impl RouteSeatsRequest {
    pub fn new(sections: Vec<crate::models::SimpleSection>, tariffs: Vec<String>, seat_class: String) -> RouteSeatsRequest {
        RouteSeatsRequest {
            sections,
            tariffs,
            seat_class,
        }
    }
}


