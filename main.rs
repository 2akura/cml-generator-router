use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr; 

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

#[derive(Debug, Serialize)]
struct Output {
    message: String,
    hostname_len: usize,
}

enum Indexer {
    ENABLE,
    HOSTNAME,
    BANNER,
    PASSWORD,
    DOMAINNAME,
} 

// This function is called from JS
//pass the destruc struc as their own variable
#[wasm_bindgen]
pub fn takejson(somejson: &str) {

    let cfg: Config = serde_json::from_str(somejson).unwrap();
    let Config { enable, hostname, banner, password, domain_name } = cfg;
    

    //for every !x.is_empty push as an aftermath into a vec
    //do this rigorously for every variable so each of em could be distinguished for translate into token process
    let mut preserved: Vec<(String, String)> = Vec::new();
    
    if let Some(e) = enable {
        if e == true {
            preserved.push(("enable".to_string(), e.to_string()));
        }
    }
    
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
    let give = take_and_return(preserved); //give the vec
    let parsing = give.as_str(); //parse String type into &str type
    let token_input = parsing; // recieve the parsed into &str un_vec
    match token_input.parse::<Indexer>() { 
        Ok(variant) => {
            let start_index = dispatching(variant);    // one; start_index is initialised and is passed to dispatching
        }
        Err(_) => panic!("Invalid token!"),
    }
}

fn take_and_return(the_vec: Vec<(String, String)>) ->String{
    
    let keys: Vec<String> = the_vec
    .into_iter()
    .map(|(k, _v)| k)  
    .collect();
    
    let un_vec:String = keys.into_iter().next().unwrap();
    un_vec
    //note this function expect return to takejson at this line
}

impl FromStr for Indexer { 
    type Err = ();         

    fn from_str(s: &str) -> Result<Self, Self::Err> { //return the checking with the standard Result enum
        match s {
            "enable" => Ok(Indexer::ENABLE),
            "hostname" => Ok(Indexer::HOSTNAME),
            "banner" => Ok(Indexer::BANNER),
            "password" =>Ok(Indexer::PASSWORD),
            "domain_name" =>Ok(Indexer::DOMAINNAME), 
            _ => Err(()), //erro handler
        }
    }
}

fn dispatching(indices: Indexer) -> u8 { // this is to pattern mathcing the index with the enum's variant

    // two; dispatching starts owning start_index here
    
    match indices {
        Indexer::ENABLE => 2,
        Indexer::HOSTNAME => 1,
        Indexer::BANNER => 3,
        Indexer::PASSWORD => 4,
        Indexer::DOMAINNAME => 5,
    }
    
    // three; start_index is returned to takejson at here
    
} // preload initialisation's process ends here, and the data is safe to be passed to the fsm

fn main (){}
