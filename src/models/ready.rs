use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyConfig {
  pub api_endpoint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyData {
  pub config: ReadyConfig
}
