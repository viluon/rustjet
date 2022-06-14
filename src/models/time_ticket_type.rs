/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeTicketType {
    #[serde(rename = "FLEX")]
    FLEX,
    #[serde(rename = "DAY")]
    DAY,
    #[serde(rename = "WEEK")]
    WEEK,
    #[serde(rename = "MONTH")]
    MONTH,

}

impl ToString for TimeTicketType {
    fn to_string(&self) -> String {
        match self {
            Self::FLEX => String::from("FLEX"),
            Self::DAY => String::from("DAY"),
            Self::WEEK => String::from("WEEK"),
            Self::MONTH => String::from("MONTH"),
        }
    }
}

impl Default for TimeTicketType {
    fn default() -> TimeTicketType {
        Self::FLEX
    }
}



