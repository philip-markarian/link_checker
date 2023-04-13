use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::{Instant, Duration};
use url;


use csv::WriterBuilder;

use reqwest::{Client, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get the input and output file paths from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: link-checker <input-file> <output-file> <number-of-columns>");
        return Ok(());
    }
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    let num_of_columns = args[3].parse::<usize>()?;

    // Create a Reqwest client
    let client = Client::new();

    // Open the input CSV file and create a CSV reader
    let input_file = File::open(input_path)?;
    let input_reader = BufReader::with_capacity(1024 * 1024, input_file);
    let mut input_csv = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input_reader);

    // Create a CSV writer for the output file
    let output_file = File::create(output_path)?;
    let output_writer = BufWriter::new(output_file);
    let mut output_csv = WriterBuilder::new()
        .has_headers(true)
        .from_writer(output_writer);

    // Write the headers for the output CSV file
    let mut headers = vec![];
    for i in 1..=num_of_columns {
        headers.push(format!("Link {}", i));
        headers.push(format!("Link {} Status Code", i));
        headers.push(format!("Link {} Redirect", i));
    }
    output_csv.write_record(&headers)?;

    let start = Instant::now();

    let mut row_count = 0;
    // Loop over each row in the input CSV file
    for result in input_csv.records() {
        // Extract the URLs from the current row
        let record = result?;
        let mut row = vec![];
        for i in 0..num_of_columns {
            let link = record.get(i).unwrap().to_string();
            println!("Link {}: {}", i+1, link);

            let mut status = "ERROR".to_string();
            let mut redirect = "ERROR".to_string();
            if link.starts_with("https://") {
                // Check the status and redirect status of the link
                if let Ok(s) = check_status(&client, &link).await {
                    status = s.to_string();
                } else {
                    println!("Error checking status for Link {}: {}", i+1, link);
                }
                if let Ok(r) = check_redirect(&client, &link).await {
                    redirect = r.to_string();
                } else {
                    println!("Error checking redirect for Link {}: {}", i+1, link);
                }
            }
            row.push(link);
            row.push(status);
            row.push(redirect);

        }
        // Write the URLs and their status and redirect status to the output CSV file
        output_csv.write_record(&row)?;

        row_count += 1;
    }
    
    let duration = start.elapsed();
    println!("Processed {} rows in {:?}", row_count, duration);
    
    Ok(())
}




#[derive(Debug)]
struct MyError {
    status_code: StatusCode,
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (status code: {})", self.message, self.status_code)
    }
}

impl Error for MyError {}

async fn check_status(client: &Client, url: &str) -> Result<u16, Box<dyn Error>> {
    println!("Checking status for {}", url);
    match tokio::time::timeout(Duration::from_secs(5), client.head(url).send()).await {
        Ok(Ok(response)) => {
            println!("Status: {:?}", response);
            Ok(response.status().as_u16())
        }
        _ => {
            println!("Request timed out or failed");
            Err(Box::new(MyError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Request timed out or failed".into(),
            }) as Box<dyn Error>)
        }
    }
}


async fn check_redirect(client: &reqwest::Client, url: &str) -> Result<String, Box<dyn Error>> {
    println!("Checking redirect for {}", url);
    match tokio::time::timeout(std::time::Duration::from_secs(5), client.head(url).send()).await {
        Ok(Ok(response)) => {
            println!("Redirect: {}", response.status().as_u16());
            if let Some(location) = response.headers().get("location") {
                let redirect_url = url::Url::parse(location.to_str()?)
                    .map_err(|e| Box::new(e) as Box<dyn Error>)?;
                Ok(redirect_url.to_string())
            } else {
                Ok("".to_string())
            }
        }
        _ => {
            println!("Request timed out or failed");
            Err(Box::new(MyError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Request timed out or failed".into(),
            }) as Box<dyn Error>)
        }
    }
}



