/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */

/// CustomerActions : Defines which actions can be executed with current ticket state



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomerActions {
    #[serde(rename = "showDetail")]
    pub show_detail: bool,
    #[serde(rename = "pay")]
    pub pay: bool,
    #[serde(rename = "payRemaining")]
    pub pay_remaining: bool,
    #[serde(rename = "evaluate")]
    pub evaluate: bool,
    #[serde(rename = "cancel")]
    pub cancel: bool,
    #[serde(rename = "storno")]
    pub storno: bool,
    #[serde(rename = "rebook")]
    pub rebook: bool,
    #[serde(rename = "editPassengers")]
    pub edit_passengers: bool,
    #[serde(rename = "additionalServices")]
    pub additional_services: bool,
    #[serde(rename = "sentToMail")]
    pub sent_to_mail: bool,
    #[serde(rename = "printTicket")]
    pub print_ticket: bool,
    #[serde(rename = "printInvoice")]
    pub print_invoice: bool,
}

impl CustomerActions {
    /// Defines which actions can be executed with current ticket state
    pub fn new(show_detail: bool, pay: bool, pay_remaining: bool, evaluate: bool, cancel: bool, storno: bool, rebook: bool, edit_passengers: bool, additional_services: bool, sent_to_mail: bool, print_ticket: bool, print_invoice: bool) -> CustomerActions {
        CustomerActions {
            show_detail,
            pay,
            pay_remaining,
            evaluate,
            cancel,
            storno,
            rebook,
            edit_passengers,
            additional_services,
            sent_to_mail,
            print_ticket,
            print_invoice,
        }
    }
}


