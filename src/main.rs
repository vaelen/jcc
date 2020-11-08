/*

Copyright (c) 2020 Andrew C. Young

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
OR OTHER DEALINGS IN THE SOFTWARE.

*/

extern crate reqwest;
extern crate encoding;

use encoding::all::WINDOWS_31J;
use encoding::{Encoding, DecoderTrap};
use reqwest::blocking::get;
use std::collections::BTreeMap;

pub const CITIES_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcc-list.txt";
pub const GUN_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcg-list.txt";
pub const KU_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/ku-list.txt";

pub const REMOTE_ENCODING: &dyn Encoding = WINDOWS_31J;

#[derive(Debug)]
struct Prefecture {
    code: String,
    name: String,
    regions: BTreeMap<String,Region>,
}

#[derive(Debug)]
struct Region {
    code: String,
    name: String,
    active: bool,
    valid_until: String,
}

fn fetch(url: &str, encoding: &dyn Encoding) -> Result<String, String> {
    match get(url) {
        Ok(resp) => match resp.bytes() {
            Ok(bytes) => match encoding.decode(&bytes, DecoderTrap::Strict) {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

fn parse(data: &str, _prefectures: &mut BTreeMap<String,Prefecture>) {
    for line in data.lines() {
        println!("{:?}", line);
    }
}

fn main() -> Result<(), String> {

    let mut prefectures: BTreeMap<String,Prefecture> = BTreeMap::new();
     
    print!("Fetching Cities List... "); 
    let cities = fetch(CITIES_URL, REMOTE_ENCODING)?;
    print!("Parsing... ");
    parse(&cities, &mut prefectures);
    println!("Done");

    print!("Fetching Gun List... "); 
    let guns = fetch(GUN_URL, REMOTE_ENCODING)?;
    print!("Parsing... ");
    parse(&guns, &mut prefectures);
    println!("Done");

    print!("Fetching Ku List... "); 
    let kus = fetch(KU_URL, REMOTE_ENCODING)?;
    print!("Parsing... ");
    parse(&kus, &mut prefectures);
    println!("Done");

    Ok(())
}
