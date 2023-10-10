use serde::Deserialize;
use tungstenite::{connect, Message};
use ureq::json;
use url::Url;

use crate::{errors::JorisError, Joris, URL_BASE, USER_AGENT};

/// To get the token of an account with two factor auth, we need something called a
/// "ticket"
pub fn get_two_factor_authentication_ticket(
    email: &str,
    password: &str,
) -> Result<String, JorisError> {
    #[derive(Deserialize)]
    struct Response {
        ticket: String,
    }

    let response = ureq::post(&format!("{}/auth/login", URL_BASE))
        .set("User-Agent", USER_AGENT)
        .send_json(json!({"login": email, "password": password}))?
        .into_json::<Response>()?;

    Ok(response.ticket)
}

impl Joris {
    /// Creates `Joris` for account with two factor authentication
    /// # Arguments
    /// - **ticket**: See the `get_two_factor_authentication_ticket` function
    /// - **auth_code**: The code from your two factor auth code provider (e.g. "Google
    /// Authenticator", "Aegis", etc...)
    pub fn new_with_ticket(ticket: String, auth_code: String) -> Result<Self, JorisError> {
        #[derive(Deserialize)]
        struct Response {
            token: String,
        }

        let response = ureq::post(&format!("{}/auth/mfa/totp", URL_BASE))
            .set("User-Agent", USER_AGENT)
            .send_json(json!({"code": auth_code, "ticket": ticket}))?
            .into_json::<Response>()?;

        let (mut socket, _) =
            connect(Url::parse("wss://gateway.discord.gg/?encoding=json&v=9").unwrap())?;

        match socket.write(Message::Text(
            json!({"d": {"token": response.token}}).to_string(),
        )) {
            Ok(_) => {}
            Err(_) => return Err(JorisError::CouldNotConnect),
        }

        Ok(Joris {
            token: response.token,
            socket,
        })
    }
}
