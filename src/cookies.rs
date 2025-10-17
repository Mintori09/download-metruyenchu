use dotenv::dotenv;
use headless_chrome::protocol::cdp::Network::CookieParam;
use std::collections::HashMap;
use std::env;

fn read_from_env() -> HashMap<String, String> {
    dotenv().ok();

    let mut map = HashMap::new();

    let access_token = env::var("accessToken").expect("Missing environment variable: accessToken");
    let me_truyen_chu_session = env::var("me_truyen_chu_session")
        .expect("Missing environment variable: me_truyen_chu_session");
    let xsrf_token = env::var("XSRF_TOKEN").expect("Missing environment variable: XSRF_TOKEN");
    let cf_clearance =
        env::var("cf_clearance").expect("Missing environment variable: cf_clearance");

    map.insert("accessToken".to_string(), access_token);
    map.insert("me_truyen_chu_session".to_string(), me_truyen_chu_session);
    map.insert("XSRF-TOKEN".to_string(), xsrf_token);
    map.insert("cf_clearance".to_string(), cf_clearance);

    map
}

pub fn create_cookies(url: &str) -> Vec<CookieParam> {
    let cookies_map = read_from_env();

    let cookies = vec![
        CookieParam {
            name: "accessToken".into(),
            value: cookies_map["accessToken"].to_string(),
            url: Some(url.into()),
            domain: None,
            path: Some("/".into()),
            secure: Some(true),
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        },
        CookieParam {
            name: "me_truyen_chu_session".into(),
            value: cookies_map["me_truyen_chu_session"].to_string(),
            url: Some(url.into()),
            domain: None,
            path: Some("/".into()),
            secure: Some(true),
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        },
        CookieParam {
            name: "XSRF-TOKEN".into(),
            value: cookies_map["XSRF-TOKEN"].to_string(),
            url: Some(url.into()),
            domain: None,
            path: Some("/".into()),
            secure: Some(true),
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        },
        CookieParam {
            name: "cf_clearance".into(),
            value: cookies_map["cf_clearance"].to_string(),
            url: Some(url.into()),
            domain: None,
            path: Some("/".into()),
            secure: Some(true),
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        },
    ];
    cookies
}

#[test]
fn print_value() {
    let cookies_map = read_from_env();
    for (k, v) in cookies_map.iter() {
        println!("{}: {}", k, v);
    }
}
