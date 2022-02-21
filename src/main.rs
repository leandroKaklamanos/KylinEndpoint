#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use edi::parse;
use edi::loose_parse;
use rocket_contrib::json::{Json, JsonValue};
use std::fs::read_to_string;
use serde::{Serialize, Deserialize};
use serde_json;


//Doing this in rust is probably not the best way considering there are much better medical info parsers on python than EDI. Too late now :L

#[derive(Serialize)]
struct EdiDocument { /* .. */ }



fn index() -> Json<&'static str> {
    Json("Hello, world!")
}

#[get("/")]
fn convert_edi() -> String {
    let edi_file_path = format!("{}/sample_edi.txt", env!("CARGO_MANIFEST_DIR"));
    let edi_string = read_to_string(edi_file_path).unwrap();
    let edi_document = loose_parse(&edi_string).unwrap();

    let serialized = serde_json::to_string(&edi_document).unwrap();
    serialized
}   

fn main() {
    convert_edi();
    rocket::ignite().mount("/", routes![convert_edi]).launch();
}
