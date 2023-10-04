use serde::Deserialize;
use ureq::json;

use crate::{errors::JorisError, URL_BASE};

pub fn get_two_factor_authentication_ticket(
    email: &str,
    password: &str,
) -> Result<String, JorisError> {
    #[derive(Deserialize)]
    struct Response {
        ticket: String,
    }

    let response = ureq::post(&format!("{}/auth/login", URL_BASE))
        .send_json(json!({"login": email, "password": password}))?
        .into_json::<Response>()?;

    Ok(response.ticket)
}
