use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

enum Response {
    Next(String),
    Done(String),
}

fn build_url(number: &str) -> String {
    format!(
        "http://www.pythonchallenge.com/pc/def/linkedlist.php?nothing={}",
        number
    )
}

async fn get_next_number(number: &str) -> Result<Response, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"and the next nothing is (\d+)").unwrap();
    }

    let resp = reqwest::get(build_url(&number)).await?.text().await?;

    if let Some(cap) = RE.captures(&resp).and_then(|x| x.get(1)) {
        let next = cap.as_str();

        return Ok(Response::Next(next.into()));
    }

    if resp == "Yes. Divide by two and keep going." {
        let parsed = number.parse::<i32>()?;
        let next = (parsed / 2).to_string();
        return Ok(Response::Next(next));
    }

    return Ok(Response::Done(resp));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut number = String::from("12345");

    loop {
        let result = get_next_number(&number).await?;
        match result {
            Response::Done(url) => {
                println!("{}", url);
                break;
            }
            Response::Next(next) => number = next,
        };
    }

    Ok(())
}
