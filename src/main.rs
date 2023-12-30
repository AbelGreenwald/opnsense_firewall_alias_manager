#[allow(unused_variables)]

use env_logger::Env;
use log::debug;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use url::{Url};
use serde::{Serialize, Deserialize};
use reqwest::header::CONTENT_TYPE;


//"{\"total\":2,\"rowCount\":-1,\"current\":1,\"rows\":[{\"ip\":\"1.1.1.1\"},{\"ip\":\"1.2.3.67\"}]}"

#[derive(Debug, Serialize, Deserialize)]
struct Obj {
  total: i32,
  rowCount: i32,
  current: i32,
  rows: Vec<Ips>
}

#[derive(Debug, Serialize, Deserialize)]
struct Ips {
  ip: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args: Vec<String> = env::args().collect();
    let user_name = env::var("OPNSENSE_USERNAME").expect("OPNSENSE_USERNAME is not set and required");
    let password: Option<String> = Some(env::var("OPNSENSE_PASSWORD").expect("OPNSENSE_PASSWORD is not set and required"));

    debug!("Arguments recieved: {}",(args.len() - 1));
    if args.len() != 4 && args.len() != 3 {
        panic!("2 or 3 command line params required: [action] [index] (ip)");
    };

    let command = args[1].to_string();
    let alias = args[2].to_string();
    let mut map = HashMap::new();
    let client = Client::new();

    match command.as_str() {
      "add" | "delete" => {
        if args.len() != 4 {
          panic!("3 command line params required for [add|delete] action: [action] [index] [ip]");
        };
        let ip = args[3].to_string();
        let url_path = build_url_path(&command,&alias);
        map.insert("address", ip);

        for (key, value) in &map {
          println!("{}: {}", key, value);
        }
        debug!("Sending to {} as {}", url_path, user_name);
        let res = client.post(url_path)
        .json(&map)
        .basic_auth(user_name, password)
        .send();
        debug!("{:?}",res.unwrap());
        //TODO: Add responds debug message
      },
      /*
        Note:  Flush API call is not persistant, rewrote to append all listed IP's to 'delete' 
      */
      "flush" | "list" => {
        let url_path = build_url_path(&command,&alias);
        map.insert("alias", alias);
        for (key, value) in &map {
          println!("{}: {}", key, value);
        }
        debug!("Sending to {} as {}", url_path, user_name);
        let res: Obj = client.post(url_path)
        .json(&map)
        .basic_auth(user_name, password)
        .header(CONTENT_TYPE, "application/json")
        .send()?
        .json()?;
        for row in &res.rows {
          println!("{}", row.ip);
        }
      },
      
      &_ => panic!("Unknown command given: {}", command)
    }
  Ok(())
}

fn build_url_path(command: &str, alias: &str) -> String
{
    let base_url = Url::parse("https://opnsense.abelswork.net/api/firewall/alias_util/");
    let api_url = base_url.unwrap().join(&[command, "/", alias].concat());
    return api_url.unwrap().as_str().to_string()
}
