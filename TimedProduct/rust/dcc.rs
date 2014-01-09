#[crate_id = "dcc"];
extern mod http;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use std::os;
use std::str;
use std::int;
use std::from_str;
mod charclass;
mod parse;


// fn main() {
//     format!("{}", Get);
//     let args = os::args();
//     match args.len() {
//         0 => unreachable!(),
//         2 => make_and_print_request(args[1]),
//         _ => {
//             println!("Usage: {} URL", args[0]);
//             return;
//         },
//     };
// }

fn main() {
    let url = "http://directus.darkscience.net:6789/";
    let request  = RequestWriter::new(Get, from_str(url).expect("Invalid URL :-("));
    
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("Service may be down"),
    };
    
    let mut body = response.read_to_end();
    let content = str::from_utf8(body);
    let v: ~[&str] = content.words().collect();
    // let a = v[11];
    // let a1: int = from_str(a);
    // let answer = int::from_str(v[11]) * int::from_str(v[13]); 
    println(v[11]);
    println(content);   
}
