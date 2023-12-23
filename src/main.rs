use reqwest::blocking::Client;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use url::{Url};
use env_logger::Env;
use log::debug;


/*
curl \
--header "Content-Type: application/json" \
--basic \
--user "token:key" \
--request POST \
--insecure \
--verbose \
--data  '{"address":"100.1.1.1"}' \
https://opnsense.abelswork.net/api/firewall/alias_util/delete/fail2ban
*/

fn main() -> Result<(), Box<dyn Error>> {

    // check if flush and other validations
    // note opnsense fortunately ignores duplicate posts

    //if flush &args[1]

    //Builder::new().init();
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    //env_logger::Builder::target(Target::Stdout);

    let args: Vec<String> = env::args().collect();

    let url_path = build_url_path(&args[1],"fail2ban"); //add or delete

    let user_name = env::var("OPNSENSE_USERNAME").unwrap();

    let password: Option<String> = Some(env::var("OPNSENSE_PASSWORD").unwrap().to_string());

    let mut map = HashMap::new();
    map.insert("address", &args[2]);

    let client = Client::new();
    let res = client.post(url_path)
      .json(&map)
      .basic_auth(user_name, password)
      .send();

    debug!("{:?}", res.unwrap());
      //match res.status() {
      //  reqwest::StatusCode::OK => {
      //      //info!("opnsense api call succeeded: {:?}", res.json())
      //  }
      //}

      //println!("Result: {:?}",res);
      //let response = match res {
      //  Ok(res) => {info!("opnsense api call succeeded: {}", &res)},
      //  Err(res) => {error!("opnsense api call failed: {}", res)}
      //};

      Ok(())
}

fn build_url_path(command: &str, alias: &str) -> String
{
    let base_url = Url::parse("https://opnsense.abelswork.net/api/firewall/alias_util/");
    let api_url = base_url.unwrap().join(&[command, "/", alias].concat());
    //println!("built {}", &api_url.clone().unwrap().as_str());

    return api_url.unwrap().as_str().to_string()
}

//Inputs
//##get inputs:
//#token user
//#token secret
//#opnsense URL
//#alias name
//
//##methods:
//check address: bool
//set address
//delete address

/*

These commands work
curl \
  --header "Content-Type: application/json" \
  --basic \
  --request POST \
  --insecure \
  --verbose \
  --data  '{"alias":"fail2ban"}' \
  https://opnsense.abelswork.net/api/firewall/alias_util/list/fail2ban

curl \
  --header "Content-Type: application/json" \
  --basic \
  --request POST \
  --insecure \
  --verbose \
  --data  '{"alias":"fail2ban"}' \
  https://opnsense.abelswork.net/api/firewall/alias_util/flush/fail2ban
*/