use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::{collections::HashMap, env::VarError};

mod jobrequestjson;
use crate::jobrequestjson::job_request_json;

#[tokio::main]
async fn main() -> Result<(), VarError> {
    let url = std::env::var("WEBSITE")?;
    println!("API Endpoint: {:?}", url);

    // Request JSON
    let body = job_request_json(100);

    // Build a post to their firebases
    let client = Client::new();
    let response = client.post(url).json(&body).send().await;

    // Make a hashmap to store the frequency of all words
    let mut map: HashMap<String, u32> = HashMap::new();

    // Make a regex for future use
    let re = Regex::new(r"[^A-z ]|[\[-`]").unwrap();

    // Attempt to deserialize the jsonData request
    if let Ok(r) = response {
        let json_data = r.json::<Box<[DocumentRequest]>>().await;
        let document = json_data.unwrap();
        for job in document.into_iter() {
            if let Some(s) = job.document.fields.description.stringValue.clone() {
                // Replace # and \n with spaces, then slice into an array of words
                let words: Vec<String> = re
                    .replace_all(&s, " ")
                    .to_string()
                    .to_lowercase()
                    .split_whitespace()
                    .map(str::to_string)
                    .collect();

                // Add to hashmap
                for word in words {
                    // Remove all of the ones that are shorter than 3
                    if word.len() <= 3 {
                        continue;
                    }

                    // Add to the map
                    match map.get(&word) {
                        Some(v) => {
                            map.insert(word.clone(), v + 1);
                        }
                        None => {
                            map.insert(word.clone(), 1);
                        }
                    }
                }
            }
        }

        // Sort hashmap into array
        let mut wordVec: Vec<(String, u32)> = map.into_iter().map(|f| f).collect();
        wordVec.sort_by(|a, b| a.1.cmp(&b.1)); 

        for w in wordVec {
            if isBlacklisted(&w.0) {
                continue;
            }
            println!("{}: {}", w.1, w.0);
        }
    } else {
        println!("Response had error!");
        return Ok(());
    }

    Ok(())
}

fn isBlacklisted(w: &String) -> bool {
    w == "with" || w == "that" || w == "will" || w == "your" || w == "have"
}

#[derive(Debug, Deserialize)]
struct DocumentRequest {
    document: RustJobDocument,
    readTime: String,
}

#[derive(Debug, Deserialize)]
struct RustJobDocument {
    name: String,
    fields: DocumentFields,
    createTime: String,
    updateTime: String,
}

// These aren't all of the fields, but they're the only ones I care about
#[derive(Debug, Deserialize)]
struct DocumentFields {
    description: StringValue,
}

#[derive(Deserialize, Debug)]
struct StringValue {
    stringValue: Option<String>,
}
