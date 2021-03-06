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
pub enum TransactionMethod {
    #[serde(rename = "ACCOUNT")]
    ACCOUNT,
    #[serde(rename = "CASH")]
    CASH,
    #[serde(rename = "BANK_TRANSFER")]
    BANKTRANSFER,
    #[serde(rename = "GIFT_CERTIFICATE")]
    GIFTCERTIFICATE,
    #[serde(rename = "CREDIT_CARD")]
    CREDITCARD,
    #[serde(rename = "ONLINE_PAYMENT")]
    ONLINEPAYMENT,

}

impl ToString for TransactionMethod {
    fn to_string(&self) -> String {
        match self {
            Self::ACCOUNT => String::from("ACCOUNT"),
            Self::CASH => String::from("CASH"),
            Self::BANKTRANSFER => String::from("BANK_TRANSFER"),
            Self::GIFTCERTIFICATE => String::from("GIFT_CERTIFICATE"),
            Self::CREDITCARD => String::from("CREDIT_CARD"),
            Self::ONLINEPAYMENT => String::from("ONLINE_PAYMENT"),
        }
    }
}

impl Default for TransactionMethod {
    fn default() -> TransactionMethod {
        Self::ACCOUNT
    }
}




