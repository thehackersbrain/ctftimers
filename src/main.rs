use clap::Parser;
use chrono::Datelike;
use select::document::Document;
use select::predicate::{Class, Name};
use reqwest::blocking::Client;
use reqwest::header;
use prettytable::{Table, Row, Cell};
use std::collections::HashMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// prints the events of next month
    #[arg(short, long)]
    next: bool,

    /// prints all the events that are available
    #[arg(short, long)]
    all: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let months = HashMap::from([
        (1, "Jan"),
        (2, "Feb"),
        (3, "Mar"),
        (4, "Apr"),
        (5, "May"),
        (6, "Jun"),
        (7, "Jul"),
        (8, "Aug"),
        (9, "Sept"),
        (10, "Oct"),
        (11, "Nov"),
        (12, "Dec"),
    ]);

    let month:String;// = chrono::Local::now().month();

    if cli.next {
        let month_num = chrono::Local::now().month();
        month = months.get(&(month_num + 1)).unwrap().to_string();
    } else {
        month = months.get(&chrono::Local::now().month()).unwrap().to_string();
    }

    let body_result = get_data();
    let body = match body_result {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Failed to fetch data: {}", e);
            return Err(e.into());
        }
    };
    // parse the HTML using select.rs
    let document = Document::from(body.as_str());

    // find the table that contains the CTF information
    let table = document.find(Class("table")).next().unwrap();

    let mut names: Vec<String> = Vec::new();
    let mut dates: Vec<String> = Vec::new();
    let mut styles: Vec<String> = Vec::new();
    let mut locations: Vec<String> = Vec::new();
    let mut weights: Vec<String> = Vec::new();

    // iterate through the rows of the table and extract the CTF information
    for row in table.find(Name("tr")).skip(1) {
        if cli.all {
            names.push(row.find(Name("a")).next().unwrap().text());
            dates.push(row.find(Name("td")).skip(1).next().unwrap().text());
            styles.push(row.find(Name("td")).skip(2).next().unwrap().text());
            locations.push(row.find(Name("td")).skip(3).next().unwrap().text().trim().to_owned());
            weights.push(row.find(Name("td")).skip(4).next().unwrap().text());
        } else {
            let date = row.find(Name("td")).skip(1).next().unwrap().text();
            if date.contains(&month) {
                names.push(row.find(Name("a")).next().unwrap().text());
                dates.push(row.find(Name("td")).skip(1).next().unwrap().text());
                styles.push(row.find(Name("td")).skip(2).next().unwrap().text());
                locations.push(row.find(Name("td")).skip(3).next().unwrap().text().trim().to_owned());
                weights.push(row.find(Name("td")).skip(4).next().unwrap().text());
            } else {
                continue;
            }
        }
    }

    print_data(names, dates, styles, locations, weights);

    Ok(())
}

fn get_data() -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .user_agent("My Rust Application")
        .build()?;

    let response = client.get("https://ctftime.org/event/list/upcoming")
        .header(header::ACCEPT_LANGUAGE, "en-US")
        .send()?;
    let body = response.text()?;
    Ok(body)
}

fn print_data(
    names: Vec<String>,
    dates: Vec<String>,
    styles: Vec<String>,
    locations: Vec<String>,
    weights: Vec<String>
    ) {
    let mut table = Table::new();
    let mut index = 0;
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("From - To"),
        Cell::new("Style"),
        Cell::new("Location"),
        Cell::new("Weight")
    ]));
    for _ in &names {
        table.add_row(Row::new(vec![
            Cell::new(&names[index]),
            Cell::new(&dates[index]),
            Cell::new(&styles[index]),
            Cell::new(&locations[index]),
            Cell::new(&weights[index]),
        ]));
        index += 1;
    }

    table.printstd();
}
