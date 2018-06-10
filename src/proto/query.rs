use curl::easy::Easy;
use serde_json::{self, Error};

use std::str;


type URL = String;

#[derive(Serialize, Deserialize, Debug)]
pub (in crate) struct QueryNumbers {
      pub (in crate) length  : usize 
    , pub (in crate) data    : Vec<usize>
    , pub (in crate) success : bool
}

pub (in crate) fn run (url : URL) -> Result<QueryNumbers, Error> {
    let mut response = Vec::new();
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|b| {
            response.extend_from_slice(b);
            Ok(b.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    info!("[*] response: {:#?}", response);

    let body = str::from_utf8(&response).unwrap_or_else(|e| {
        panic!("Failed to parse response from {} -- {}", url, e);
    });

    info!("[#] body: {}", body);

    let numbers : QueryNumbers = serde_json::from_str(body).unwrap_or_else(|e| {
        panic!("Failed to parse json -- {}", e);
    });

    info!("[#] numbers: {:#?}", numbers);

    Ok(numbers)
}