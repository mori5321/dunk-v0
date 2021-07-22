use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Request {
    base_url: String,
    path: String,
    method: String, // TODO: Enumにしたい。dhall側にrustのenum型を渡してあげる必要がありそう
    headers: HashMap<String, String>,
    req_body: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    status: u16,
    body: serde_json::Value,
}

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();
    let opt_dhall_path = args.get(1);
    let dhall_path = opt_dhall_path.unwrap();

    let req: Request = serde_dhall::from_file(dhall_path).parse().unwrap();

    let url = req.base_url + req.path.as_str();

    let response = match req.method.as_str() {
        "GET" => {
            let mut get_request = ureq::get(&url);
            for (header, value) in req.headers.into_iter() {
                get_request = get_request.set(&header, &value);
            }
            get_request.call()?
        }
        "POST" => ureq::post(&url).send_json(req.req_body.unwrap())?,
        _ => panic!("Invalid Method type."),
    };

    let resp = Response {
        status: response.status(),
        body: response.into_json::<serde_json::Value>()?,
    };

    let resp_json = serde_json::to_string(&resp).unwrap();

    println!("{}", resp_json);

    Ok(())
}
