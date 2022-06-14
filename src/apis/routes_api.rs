/*
 * RegioJet API
 *
 * A set of endpoints to interact with RegioJet transport services. Search for connections, book tickets, see the list of served stations and more. All endpoints consume and produce JSON strings, with the exception of ticket printing (/tickets/{ticketId}/print) that produce printable HTML code.
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: developers@studentagency.cz
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_departures`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetDeparturesError {
    Status400(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_passengers_data`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPassengersDataError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_route_free_seats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRouteFreeSeatsError {
    Status400(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_simple_route_detail`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSimpleRouteDetailError {
    Status400(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_routes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRoutesError {
    Status400(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`simple_search_routes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleSearchRoutesError {
    Status400(crate::models::ErrorResponse),
    Status404(crate::models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn get_departures(configuration: &configuration::Configuration, station_id: i64, x_lang: Option<&str>, limit: Option<i32>) -> Result<crate::models::ArrivalDepartureConnection, Error<GetDeparturesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/{stationId}/departures", local_var_configuration.base_path, stationId=station_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetDeparturesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_passengers_data(configuration: &configuration::Configuration, route_id: &str, filter: crate::models::PassengersDataRequest, x_lang: Option<&str>, x_currency: Option<&str>) -> Result<crate::models::PassengersDataResponse, Error<GetPassengersDataError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/{routeId}/passengersData", local_var_configuration.base_path, routeId=crate::apis::urlencode(route_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_currency {
        local_var_req_builder = local_var_req_builder.header("X-Currency", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&filter);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetPassengersDataError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_route_free_seats(configuration: &configuration::Configuration, route_id: &str, request: crate::models::RouteSeatsRequest, x_lang: Option<&str>) -> Result<crate::models::RouteSeatsResponse, Error<GetRouteFreeSeatsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/{routeId}/freeSeats", local_var_configuration.base_path, routeId=crate::apis::urlencode(route_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRouteFreeSeatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_simple_route_detail(configuration: &configuration::Configuration, route_id: &str, from_station_id: i64, to_station_id: i64, x_lang: Option<&str>, x_currency: Option<&str>, tariffs: Option<Vec<String>>) -> Result<crate::models::Route, Error<GetSimpleRouteDetailError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/{routeId}/simple", local_var_configuration.base_path, routeId=crate::apis::urlencode(route_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("fromStationId", &from_station_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("toStationId", &to_station_id.to_string())]);
    if let Some(ref local_var_str) = tariffs {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tariffs".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tariffs", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_currency {
        local_var_req_builder = local_var_req_builder.header("X-Currency", local_var_param_value.to_string());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetSimpleRouteDetailError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Authorization not required (filtres info bubbles)
pub async fn search_routes(configuration: &configuration::Configuration, from_location_id: i64, from_location_type: &str, to_location_id: i64, to_location_type: &str, x_lang: Option<&str>, x_currency: Option<&str>, departure_time: Option<String>, tariffs: Option<Vec<String>>, action_price: Option<&str>) -> Result<crate::models::SearchResult, Error<SearchRoutesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/search", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("fromLocationId", &from_location_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("fromLocationType", &from_location_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("toLocationId", &to_location_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("toLocationType", &to_location_type.to_string())]);
    if let Some(ref local_var_str) = departure_time {
        local_var_req_builder = local_var_req_builder.query(&[("departureTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tariffs {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tariffs".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tariffs", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = action_price {
        local_var_req_builder = local_var_req_builder.query(&[("actionPrice", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_currency {
        local_var_req_builder = local_var_req_builder.header("X-Currency", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Token-Hash", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SearchRoutesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Authorization not required (filtres info bubbles)
pub async fn simple_search_routes(configuration: &configuration::Configuration, from_location_id: i64, from_location_type: &str, to_location_id: i64, to_location_type: &str, x_lang: Option<&str>, x_currency: Option<&str>, x_used_departure_from_date_time: Option<String>, x_used_departure_to_date_time: Option<String>, departure_date: Option<String>, tariffs: Option<Vec<String>>, action_price: Option<&str>, _move: Option<&str>) -> Result<crate::models::SimpleSearchResult, Error<SimpleSearchRoutesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/routes/search/simple", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("fromLocationId", &from_location_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("fromLocationType", &from_location_type.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("toLocationId", &to_location_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("toLocationType", &to_location_type.to_string())]);
    if let Some(ref local_var_str) = departure_date {
        local_var_req_builder = local_var_req_builder.query(&[("departureDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = tariffs {
        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("tariffs".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => local_var_req_builder.query(&[("tariffs", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref local_var_str) = action_price {
        local_var_req_builder = local_var_req_builder.query(&[("actionPrice", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _move {
        local_var_req_builder = local_var_req_builder.query(&[("move", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_lang {
        local_var_req_builder = local_var_req_builder.header("X-Lang", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_currency {
        local_var_req_builder = local_var_req_builder.header("X-Currency", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_used_departure_from_date_time {
        local_var_req_builder = local_var_req_builder.header("X-Used-DepartureFromDateTime", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = x_used_departure_to_date_time {
        local_var_req_builder = local_var_req_builder.header("X-Used-DepartureToDateTime", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-Token-Hash", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SimpleSearchRoutesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

