use std::fs::File;
use std::io::{Result, Write};

use oops::Oops;
use select::document::Document;
use select::predicate::{Attr, Name};
use serde::Serialize;
use stdinix::stdinix;
use clap::{ArgGroup, StructOpt};
use ureq;

/// A rightmove property page scraper.
#[derive(StructOpt, Debug)]
#[structopt(name = "rightscrapex", group = ArgGroup::with_name("emit"))]
struct Opt {
    /// Activate filter on listings with floorplan tab
    #[structopt(short, long)]
    floorplan: bool,

    /// Emit jsonlines
    #[structopt(short, long, group = "emit")]
    json: bool,

    /// Emit urls
    #[structopt(short, long, group = "emit")]
    urls: bool,
}

#[derive(Debug, Serialize)]
struct ScrapeResult {
    url: String,
    summary: String,
    human_identifier: String,
    price: String,
    floorplan_url: Option<String>,
    location_image_url: String,
}

fn scrape(url: String, doc: Document) -> Result<ScrapeResult> {
    let summary = doc
        .find(Attr("itemprop", "streetAddress"))
        .next()
        .lazy_oops(|| format!("Missing summary for {}", &url))?
        .text()
        .trim()
        .to_owned();
    let floorplan_url = doc
        .find(Attr("href", "#/floorplan?activePlan=1"))
        .next()
        .lazy_oops(|| format!("Missing floorplans tab for {}", &url))
        .and_then(|anchor| {
            anchor
                .attr("href")
                .lazy_oops(|| format!("Missing anchor for {}", &url))
                .and_then(|href| Ok(url.clone() + &href[href.bytes().position(|v| v == b'/').oops("No / found in anchor")? + 1..]))
        })
        .ok();

    let page_model = doc
        .find(Name("script"))
        .map(|node| node.text())
        .filter(|text| text.contains("PAGE_MODEL"))
        .next()
        .oops("No page model found")?
        .split(" = ")
        .skip(1)
        .collect::<String>();

    // Data from page model.
    // Price: .propertyData.prices.primaryPrice
    // Human identifier: .propertyData.text.pageTitle
    // Location image URL: .propertyData.staticMapImgUrls.staticMapImgUrlMobile
    let model: serde_json::value::Value = serde_json::from_str(&page_model).oops("Page model couldn't be parsed as json")?;
    let price = model
        .get("propertyData")
        .oops("propertyData not found in model")?
        .get("prices")
        .oops("prices not found in model")?
        .get("primaryPrice")
        .oops("primaryPrice not found in prices")?
        .as_str()
        .oops("primaryPrice wasn't a json string")?
        .to_owned();
    let human_identifier = model
        .get("propertyData")
        .oops("propertyData not found in model")?
        .get("text")
        .oops("text wasn't found model")?
        .get("pageTitle")
        .oops("pageTitle wasn't found in text")?
        .as_str()
        .oops("pageTitle wasn't a json-string")?
        .to_owned();
    let location_image_url = model
        .get("propertyData")
        .oops("propertyData not found in model")?
        .get("staticMapImgUrls")
        .oops("staticMapImgUrls wasn't found model")?
        .get("staticMapImgUrlMobile")
        .oops("staticMapImgUrlMobile wasn't found in image urls")?
        .as_str()
        .oops("staticMapImgUrlMobile wasn't a json-string")?
        .to_owned();

    Ok(ScrapeResult {
        url,
        summary,
        human_identifier,
        price,
        floorplan_url,
        location_image_url,
    })
}

fn filter<'a, 'b>(cfg: &'a Opt, res: &'b ScrapeResult) -> Option<&'b ScrapeResult> {
    if cfg.floorplan && res.floorplan_url.is_none() {
        None
    } else {
        Some(res)
    }
}

fn main() -> Result<()> {
    let cfg = Opt::from_args();

    stdinix(|line| {
        let body = ureq::get(&line[..])
            .call()
            .oops("Failed to request")?
            .into_string()?;

        let doc = Document::from(&body[..]);

        let res = scrape(line.trim().to_owned(), doc).or_else(|err| {
            eprintln!("Dumping document to tmp.html");
            let mut file = File::create("tmp.html")?;
            file.write_all(body.as_bytes())?;
            Err(err)
        })?;

        let res = filter(&cfg, &res);

        if let Some(res) = res {
            match (cfg.json, cfg.urls) {
                (true, false) => {
                    println!("{}", serde_json::to_string(&res)?);
                    std::io::stdout().flush()
                }
                (false, true) => {
                    println!("{}", res.url);
                    std::io::stdout().flush()
                }
                _ => panic!("No emit settings!"),
            }
        } else {
            Ok(())
        }
    })
}
