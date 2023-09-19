use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SentryMessage {
  pub dsn: sentry_types::Dsn,
  pub body: String,
}

impl SentryMessage {
  pub async fn forward(&self) -> Result<(), reqwest::Response> {
    let client = reqwest::Client::new();
    let url = self.dsn.envelope_api_url().to_string() + "?sentry_key=" + self.dsn.public_key();
    let response = client
      .post(&url)
      .header("content-type", "application/x-sentry-envelope")
      .body(self.body.clone())
      .send()
      .await;

    if let Ok(response) = response && !response.status().is_success() {
      return Err(response);
    }

    Ok(())
  }
}

impl TryFrom<String> for SentryMessage {
  fn try_from(body: String) -> Result<Self, Self::Error> {
    let header = body.lines().next().ok_or(SentryError::InvalidBody)?;
    let message = serde_json::from_str::<serde_json::Value>(header).map_err(|_| SentryError::InvalidBody)?;

    if let Some(dsn) = message.get("dsn") && let Some(dsn) = dsn.as_str() && let Ok(dsn) = sentry_types::Dsn::from_str(dsn) {
      Ok(SentryMessage { dsn, body })
    } else {
      Err(SentryError::InvalidDsn)
    }
  }

  type Error = SentryError;
}

pub enum SentryError {
  InvalidDsn,
  InvalidBody,
}
