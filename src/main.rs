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

use encoding::all::WINDOWS_31J;
use encoding::{Encoding, DecoderTrap};
use regex::Regex;
use reqwest::blocking::get;
use std::collections::BTreeMap;
use std::fmt;

pub const CITY_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcc-list.txt";
pub const GUN_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/jcg-list.txt";
pub const KU_URL: &str = "https://www.jarl.org/Japanese/A_Shiryo/A-2_jcc-jcg/ku-list.txt";

pub const REMOTE_ENCODING: &dyn Encoding = WINDOWS_31J;

pub const CITY_RE: &str = r"^(?P<deleted>[*]?)\s+(?P<num>\d+)\s+(?P<name>\S+)\s+(?P<kanji>\S+)(\s+(?P<valid_until>\S+))?\s*$";
pub const GUN_RE: &str = CITY_RE;
pub const KU_RE: &str = r"^(?P<num>\d+)((?P<deleted>※)|\s+)(?P<name>\S+)\s+(?P<kanji>\S+)(\s+(?P<valid_until>\S+))?$";

#[derive(Clone, Copy, Debug)]
enum RegionType {
    City,
    Gun,
    Ku
}

impl fmt::Display for RegionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RegionType::City => "City",
            RegionType::Gun => "Gun",
            RegionType::Ku => "Ku",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
struct Region {
    code: String,
    name: String,
    deleted: bool,
    valid_until: String,
    region_type: RegionType,
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

fn parse(data: &str, re_str: &str, region_type: RegionType) -> BTreeMap<String,Region> {
    let re = Regex::new(re_str).unwrap();
    data.lines().map(|line| {
        re.captures(line).map(|caps| {
            let num = caps.name("num").unwrap().as_str();
            let name = caps.name("name").unwrap().as_str();
            let kanji = caps.name("kanji").unwrap().as_str();
            let deleted: bool = match caps.name("deleted") {
                Some(d) => !d.as_str().is_empty(),
                None => false,
            };
            let valid_until = match caps.name("valid_until") {
                Some(m) => m.as_str().to_string(),
                None => String::new(),
            };
            (num.to_string(),
             Region {
                 code: num.to_string(),
                 name: format!("{} / {}", name, kanji),
                 deleted: deleted,
                 valid_until: valid_until,
                 region_type: region_type,
             })
        })
    }).filter(|o| o.is_some()).map(|o| o.unwrap()).collect()
}

fn main() -> Result<(), String> {
    let mut regions = BTreeMap::new();
    
    eprint!("Fetching Cities List... "); 
    let cities = fetch(CITY_URL, REMOTE_ENCODING)?;
    eprint!("Parsing... ");
    regions.append(&mut parse(&cities, CITY_RE, RegionType::City));
    eprintln!("Done");

    eprint!("Fetching Gun List... "); 
    let guns = fetch(GUN_URL, REMOTE_ENCODING)?;
    eprint!("Parsing... ");
    regions.append(&mut parse(&guns, GUN_RE, RegionType::Gun));
    eprintln!("Done");

    eprint!("Fetching Ku List... "); 
    let kus = fetch(KU_URL, REMOTE_ENCODING)?;
    eprint!("Parsing... ");
    regions.append(&mut parse(&kus, KU_RE, RegionType::Ku));
    eprintln!("Done");

    eprint!("Loading Prefecture List... ");
    let prefectures = prefectures();
    eprintln!("Done");

    println!("STATE,STATE Name,CNTY,CNTY Name,Deleted,Valid Until,Type");
    for (_, region) in regions.iter() {
        let p_code: &str = &region.code[0..2];
        let p_name: &str = &prefectures.get(p_code).unwrap();
        println!("{},{},{},{},{},{},{}", p_code, p_name, region.code, region.name, region.deleted, region.valid_until, region.region_type);
    }

    Ok(())
}

fn prefectures() -> BTreeMap<String,String> {
    [("01", "Hokkaido / 北海道"),
     ("02", "Aomori / 青森"),
     ("03", "Iwate / 岩手"),
     ("04", "Akita / 秋田"),
     ("05", "Yamagata / 山形"),
     ("06", "Miyagi / 宮城"),
     ("07", "Fukushima / 福島"),
     ("08", "Niigata / 新潟"),
     ("09", "Nagano / 長野"),
     ("10", "Tokyo / 東京"),
     ("11", "Kanagawa / 神奈川"),
     ("12", "Chiba / 千葉"),
     ("13", "Saitama / 埼玉"),
     ("14", "Ibaraki / 茨城"),
     ("15", "Tochigi / 栃木"),
     ("16", "Gunma / 群馬"),
     ("17", "Yamanashi / 山梨"),
     ("18", "Shizuoka / 静岡"),
     ("19", "Gifu / 岐阜"),
     ("20", "Aichi / 愛知"),
     ("21", "Triple / 三重"),
     ("22", "Kyoto / 京都"),
     ("23", "Shiga / 滋賀"),
     ("24", "Nara / 奈良"),
     ("25", "Osaka / 大阪"),
     ("26", "Wakayama / 和歌山"),
     ("27", "Hyogo / 兵庫"),
     ("28", "Toyama / 富山"),
     ("29", "Fukui / 福井"),
     ("30", "Ishikawa / 石川"),
     ("31", "Okayama / 岡山"),
     ("32", "Shimane / 島根"),
     ("33", "Yamaguchi / 山口"),
     ("34", "Tottori / 鳥取"),
     ("35", "Hiroshima / 広島"),
     ("36", "Kagawa / 香川"),
     ("37", "Tokushima / 徳島"),
     ("38", "Ehime / 愛媛"),
     ("39", "Kochi / 高知"),
     ("40", "Fukuoka / 福岡"),
     ("41", "Saga / 佐賀"),
     ("42", "Nagasaki / 長崎"),
     ("43", "Kumamoto / 熊本"),
     ("44", "Oita / 大分"),
     ("45", "Miyazaki / 宮崎"),
     ("46", "Kagoshima / 鹿児島"),
     ("47", "Okinawa / 沖縄"),
    ].iter().map(|(k,v)| (k.to_string(), v.to_string())).collect()
}
