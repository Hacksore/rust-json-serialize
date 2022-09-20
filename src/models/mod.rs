
use serde::{ Serialize, Deserialize };
mod ready;
mod login;

use ready::*;
use login::*;

/// Includes the base props from discord
/// ex: evt, nonce
#[derive(Serialize, Deserialize, Debug)]
pub struct Based {
  pub evt: String,
  pub nonce: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
#[serde(rename_all = "lowercase")]
pub enum BasedEvents {  
  Ready {
    #[serde(flatten)]
    _default: Based,
    data: ReadyData,    
  },
  Login {
    #[serde(flatten)]
    _default: Based,
    data: LoginData,    
  },  
}
