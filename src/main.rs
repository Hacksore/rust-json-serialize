use serde_json::Result;
use serde_json::Value;
use serde_json::json;

use crate::models::BasedEvents;

mod models;

fn print_out_thing(payload: String) -> Result<()> {
    let raw_payload: Value = serde_json::from_str(&payload)?;
    let cmd = &raw_payload["cmd"];
    println!("cmd {}", cmd);

    let event = serde_json::from_str(&payload)?;
    match event {
      BasedEvents::Login { data, _default } => {
        println!("got login event, {:#?}", data);
      },
      BasedEvents::Ready { data, _default } => {
        println!("got ready event, {:#?}", data);
      },
    }

    Ok(())
}

fn main() {
    let json_data = json!({
      "cmd": "ready",
      "evt": "test",
      "nonce": "test",
      "data": {
        "config": {
          "api_endpoint": "https://lmal"
        }
      }
    });

    let s = json_data.to_string();
    print_out_thing(s).unwrap();

    let json_data = json!({
      "cmd": "login",
      "evt": "test",
      "nonce": "test",
      "data": {
        "config": {
          "test": "testlololo"
        }
      }
    });

    let s = json_data.to_string();
    print_out_thing(s).unwrap();

}
