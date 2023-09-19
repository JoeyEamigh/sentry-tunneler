#[derive(Debug, Clone)]
pub struct TunnelConfig {
  pub sentry_hosts: Option<Vec<String>>,
  pub allowed_project_ids: Option<Vec<String>>,
  pub tunnel_path: String,
  pub listen_port: u16,
  pub cors_allowed_origins: Option<Vec<String>>,
}

impl TunnelConfig {
  pub fn new() -> Self {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let sentry_hosts = if let Ok(hosts) = std::env::var("ALLOWED_SENTRY_HOSTS") {
      Some(
        hosts
          .split(',')
          .map(|s| s.replace("http://", "").replace("https://", "").to_string())
          .collect(),
      )
    } else {
      None
    };

    let allowed_project_ids = if let Ok(project_ids) = std::env::var("ALLOWED_PROJECT_IDS") {
      Some(project_ids.split(',').map(|s| s.to_string()).collect())
    } else {
      None
    };

    let cors_allowed_origins = if let Ok(origins) = std::env::var("CORS_ALLOWED_ORIGINS") {
      Some(origins.split(',').map(|s| s.to_string()).collect())
    } else {
      None
    };

    let tunnel_path = std::env::var("TUNNEL_PATH").unwrap_or("/tunnel".to_string());

    let listen_port = std::env::var("LISTEN_PORT")
      .unwrap_or("3000".to_string())
      .parse::<u16>()
      .unwrap_or(3000);

    TunnelConfig {
      sentry_hosts,
      allowed_project_ids,
      tunnel_path,
      listen_port,
      cors_allowed_origins,
    }
  }

  pub fn is_allowed_host(&self, host: &str) -> bool {
    if let Some(hosts) = &self.sentry_hosts {
      hosts.contains(&host.to_string())
    } else {
      true
    }
  }

  pub fn is_allowed_project(&self, project_id: &str) -> bool {
    if let Some(project_ids) = &self.allowed_project_ids {
      project_ids.contains(&project_id.to_string())
    } else {
      true
    }
  }
}
