/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */

/// TicketState : * `USED` - driven ticket * `VALID` - Valid and paid ticket * `UNPAID` - Unpaid reservation * `CANCELED` - Canceled by user (soft-booking) * `DELETED` - Canceled by user (hard-booking) * `EXPIRED` - Expirated soft-booking which cant be bought anymore * `TO_BE_EXPIRED` - Expirated soft-booking which werent be able to finish by system 

/// * `USED` - driven ticket * `VALID` - Valid and paid ticket * `UNPAID` - Unpaid reservation * `CANCELED` - Canceled by user (soft-booking) * `DELETED` - Canceled by user (hard-booking) * `EXPIRED` - Expirated soft-booking which cant be bought anymore * `TO_BE_EXPIRED` - Expirated soft-booking which werent be able to finish by system 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TicketState {
    #[serde(rename = "USED")]
    USED,
    #[serde(rename = "VALID")]
    VALID,
    #[serde(rename = "UNPAID")]
    UNPAID,
    #[serde(rename = "CANCELED")]
    CANCELED,
    #[serde(rename = "DELETED")]
    DELETED,
    #[serde(rename = "EXPIRED")]
    EXPIRED,
    #[serde(rename = "TO_BE_EXPIRED")]
    TOBEEXPIRED,

}

impl ToString for TicketState {
    fn to_string(&self) -> String {
        match self {
            Self::USED => String::from("USED"),
            Self::VALID => String::from("VALID"),
            Self::UNPAID => String::from("UNPAID"),
            Self::CANCELED => String::from("CANCELED"),
            Self::DELETED => String::from("DELETED"),
            Self::EXPIRED => String::from("EXPIRED"),
            Self::TOBEEXPIRED => String::from("TO_BE_EXPIRED"),
        }
    }
}

impl Default for TicketState {
    fn default() -> TicketState {
        Self::USED
    }
}




