#[crate_id = "dcc"];
extern mod http;
use http::client::RequestWriter;
use http::method::Get;
use http::headers::HeaderEnum;
use std::os;
use std::str;
use std::int;
use std::from_str;
use core::from_str::FromStr::from_str;
mod charclass;
mod parse;


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
    // let xx = v.map(|x| if *x > max { max = *x });
    let v1 = from_str::<int>(v[11]).get();
    let v2 = from_str::<int>(v[13]).get();
    // let a = v[11];
    // let a1: int = from_str(a);
    // let answer = int::from_str(v[11]) * int::from_str(v[13]); 
    println(v1);
    println(v2);
    println(content);   
}
