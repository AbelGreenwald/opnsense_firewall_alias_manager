use env_logger::Env;
use log::{debug, error, warn};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct OpnsenseResponse {
    total: i32,
    #[serde(rename = "rowCount")]
    row_count: i32,
    current: i32,
    rows: Vec<OpnsenseResponseIp>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpnsenseResponseIp {
    ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpnsenseMessageIp {
    address: String,
}

struct Credentials {
    username: String,
    password: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args: Vec<String> = env::args().collect();
    let credentials = Credentials {
        username: env::var("OPNSENSE_USERNAME").expect("OPNSENSE_USERNAME is not set and required"),
        password: Some(
            env::var("OPNSENSE_PASSWORD").expect("OPNSENSE_PASSWORD is not set and required"),
        ),
    };

    debug!("Arguments recieved: {}", (args.len() - 1));

    if args.len() != 4 && args.len() != 3 {
        error!("2 or 3 command line params required: [action] [index] (ip)");
    };

    let command = args[1].to_string();
    let alias = args[2].to_string();
    let client = Client::new();
    match command.as_str() {
        "add" | "delete" => {
            let ip = args[3].to_string();
            modify(&client, &credentials, &command, &alias, ip)
        }
        "list" => {
            let url_path = build_url_path(&command, &alias);
            list(&client, url_path, &credentials);
        }
        "flush" => {
            flush(client, credentials, &alias);
        }
        &_ => panic!("Unknown command given: {}", command),
    }
    Ok(())
}

fn modify(client: &Client, credentials: &Credentials, command: &str, alias: &str, ip: String) {
    let url_path = build_url_path(&command, &alias);

    debug!(
        "Sending to {:?} to {} as {}",
        ip, url_path, credentials.username
    );
    let post_msg: OpnsenseMessageIp = OpnsenseMessageIp { address: ip };

    let res = client
        .post(url_path)
        .json(&post_msg)
        .basic_auth(credentials.username.clone(), credentials.password.clone())
        .send();
    debug!("{:?}", res.unwrap());
}

fn list(client: &Client, url_path: String, credentials: &Credentials) -> Option<OpnsenseResponse> {
    debug!("Sending to {} as {}", url_path, credentials.username);

    let res = client
        .post(url_path)
        .json("{}")
        .basic_auth(&credentials.username, credentials.password.clone())
        .header(CONTENT_TYPE, "application/json")
        .send();

    match res {
        Ok(r) => {
            let ips = r.json::<OpnsenseResponse>().unwrap();
            print!("list operation returned: {:?}", ips);
            Some(ips)
        }
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

fn flush(client: Client, credentials: Credentials, alias: &str) {
    let list_path = build_url_path("list", alias);
    let res = list(&client, list_path, &credentials);

    match res {
        Some(ips) => {
            //coerce opnsense output into different opnsense input
            for ip in ips.rows.iter() {
                debug!("flushing {:?}", ip);
                modify(&client, &credentials, "delete", alias, ip.ip.clone())
            }
        }
        None => warn!("No Response from Server"),
    }
    //debug!("{:?}", res)
}

fn build_url_path(command: &str, alias: &str) -> String {
    let base_url = Url::parse("https://opnsense.abelswork.net/api/firewall/alias_util/");
    let api_url = base_url.unwrap().join(&[command, "/", alias].concat());
    return api_url.unwrap().as_str().to_string();
}
