use proto::query;

macro_rules! QUERY { () => ("https://qrng.anu.edu.au/API/jsonI.php?length={}&type={}&size={}") }

#[derive(StructOpt, Debug)]
pub enum QuantumNumberType {
    Uint8,
    Uint16
    // , Hex16
}

impl From<QuantumNumberType> for String {
    fn from (t : QuantumNumberType) -> Self {
        match t {
              QuantumNumberType::Uint8  => String::from("uint8")
            , QuantumNumberType::Uint16 => String::from("uint16")
            // , QuantumNumberType::Hex16  => String::from("hex16")
        }
    }
}


pub fn get_random_quantum_numbers (::Config { array_length, block_size, data_type, verbosity } : ::Config) -> Vec<usize> {
    let typ = if let Some(dt) = data_type { dt } else { QuantumNumberType::Uint16 };
    
    let url = format!(QUERY!(), array_length, String::from(typ), block_size);
    info!("[?] Query URL: {}", url);

    query::run(url)
        .expect("Error querying data")
        .data
}