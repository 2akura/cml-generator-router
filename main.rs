use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config { 
// we alr wrapped the val in Option,
//thus presence are handled from now on
//since serde will skip anything that arent found for us
    hostname: Option<String>,
    banner: Option<String>,
    password: Option<String>,
    domain_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct Output {
    message: String,
    hostname_len: usize,
}

// This function is called from JS
//pass the destruc struc as their own variable
#[wasm_bindgen]
pub fn takejson(somejson: &str) {

    let cfg: Config = serde_json::from_str(somejson).unwrap();
    let Config { hostname, banner, password, domain_name } = cfg;
    

    //for every !x.is_empty push as an aftermath into a vec
    //do this rigorously for every variable so each of em could be distinguished for translate into token process
    let mut preserved: Vec<(String, String)> = Vec::new();

    if let Some(h) = hostname {
        if !h.is_empty() {
            preserved.push(("hostname".to_string(), h));
        }
    }

    if let Some(b) = banner {
        if !b.is_empty() {
            preserved.push(("banner".to_string(), b));
        }
    }

    if let Some(p) = password {
        if !p.is_empty() {
            preserved.push(("password".to_string(), p));
        }
    }

    if let Some(d) = domain_name {
        if !d.is_empty() {
            preserved.push(("domain_name".to_string(), d));
        
        }
    }
    let give_vec = take(preserved);
    //in here we create another var, to be passed to the next fn we want to give to
 
}

fn take(the_vec: Vec<(String, String)>){}

fn main (){}
