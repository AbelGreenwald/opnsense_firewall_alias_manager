use env_logger::Env;
use log::debug;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process;
use url::{Url};

#[derive(Debug, Serialize, Deserialize)]
struct Obj {
  total: i32,
  #[serde(rename = "rowCount")]
  row_count: i32,
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
      .basic_auth(user_name.clone(), password.clone())
      .send();
      debug!("{:?}",res.unwrap());
    },
    "list" => {
      let url_path = build_url_path(&command,&alias);
      map.insert("alias", alias);
      debug!("Sending to {} as {}", url_path, user_name);
      let res: Obj = client.post(url_path)
      .json(&map)
      .basic_auth(user_name.clone(), password.clone())
      .header(CONTENT_TYPE, "application/json")
      .send()?
      .json()?;
      let mut ip_found: bool = false;
      for row in &res.rows {
        println!("{}", row.ip);
        if args.len() == 4 {
          if row.ip == args[3].to_string() {
            ip_found = true;
          }
        }
      }
      if args.len() == 4 {
        if ip_found {
          println!("IP Found: {}", args[3].to_string())
        }
        else {
          println!("IP NOT Found: {}", args[3].to_string());
          process::exit(0x001);
        }
      }
    },
    "flush" => {
      let url_path = build_url_path(&command,&alias);
      //map.insert("alias", alias);
      debug!("Sending to {} as {}", url_path, user_name);
      let res = client.post(url_path)
      .json(&map)
      .basic_auth(user_name.clone(), password.clone())
      .header(CONTENT_TYPE, "application/json")
      .send();
      debug!("{:?}", res)
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
