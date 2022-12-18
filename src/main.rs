use reqwest::Client;
use std::env::VarError;
use serde::Deserialize;

mod jobrequestjson;
use crate::jobrequestjson::job_request_json;


#[tokio::main]
async fn main() -> Result<(), VarError> {
    let url = std::env::var("WEBSITE")?;
    println!("API Endpoint: {:?}", url);

    // Request JSON
    let body = job_request_json(1);

    // Build a post to their firebase
    let client = Client::new();
    let response = client.post(url).json(&body).send().await;

    // Attempt to deserialize the jsonData request
    if let Ok(r) = response {
        // let text = r.text().await.unwrap();
        // println!("{}", text);
        let json_data = r.json::<Box<[DocumentRequest]>>().await;
        println!("{:?}", json_data);
    } else {
        println!("Response had error!");
        return Ok(());
    }



    Ok(())
}

#[derive(Debug)]
#[derive(Deserialize)]
struct DocumentRequest {
    document: RustJobDocument,
    readTime: String
}

#[derive(Debug)]
#[derive(Deserialize)]
struct RustJobDocument {
    name: String,
    fields: DocumentFields,
    createTime: String,
    updateTime: String
}

// These aren't all of the fields, 
#[derive(Debug)]
#[derive(Deserialize)]
struct DocumentFields {
    location: StringValue,
    salaryFrom: StringValue,
    salaryTo: StringValue,
    title: StringValue,
    country: StringValue,
    remote: StringValue,
    company: StringValue,
    description: StringValue
}

#[derive(Deserialize)]
#[derive(Debug)]
struct StringValue {
    stringValue: Option<String>
}