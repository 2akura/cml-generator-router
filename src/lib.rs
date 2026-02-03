use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config { 
//we alr wrapped the val in Option,
//thus presence are handled from now on
//since serde will skip anything that arent found for us
    enable:Option<bool>,
    hostname: Option<String>,
    banner: Option<String>,
    password: Option<String>,
    domain_name: Option<String>,
}

//#[serde(skip_serializing_if="Option::is_none")] is to skip the field
//if any field fo this struct can't find their own suitable input 
//since we dont know wether 
#[derive(Serialize, Debug)]
struct ToJS {
    
    #[serde(skip_serializing_if="Option::is_none")]
    en:Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    host:Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    bann:Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    passw:Option<String>,
    
    #[serde(skip_serializing_if="Option::is_none")]
    dom_name:Option<String>,
}

#[wasm_bindgen]
pub fn takejson(somejson: &str) ->JsValue {

    let cfg: Config = serde_json::from_str(somejson).unwrap();
    let Config { enable, hostname, banner, password, domain_name } = cfg;
    let mut tojs = ToJS {
        en : Some(String::new()),
        host : Some(String::new()),
        bann : Some(String::new()),
        passw : Some(String::new()),
        dom_name : Some(String::new()),
    };

    //for every !x.is_empty put into the serializing struct

    if let Some(e) = enable {
        if e == true {
            tojs.en = Some(String::from("enable"));
        }
    }
    
    if let Some(h) = hostname {
        if !h.is_empty() {
            tojs.host = Some(String::from("hostname".to_owned()+" "+&h));
        }
    }

    if let Some(b) = banner {
        if !b.is_empty() {
            tojs.bann = Some(String::from("banner".to_owned()+" "+&b));
        }
    }

    if let Some(p) = password {
        if !p.is_empty() {
            tojs.passw = Some(String::from("password".to_owned()+" "+&p));
        }
    }

    if let Some(d) = domain_name {
        if !d.is_empty() {
            tojs.dom_name = Some(String::from("domain name".to_owned()+" "+&d));
        
        }
    }
    serde_wasm_bindgen::to_value(&tojs).unwrap()
    //println!("{:?}", tojs);
    
}
