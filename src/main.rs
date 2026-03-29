use std::fs::File;
use std::io::Write;

use clap::{ArgGroup, Parser};
use eyre::{ContextCompat, WrapErr};
use select::document::Document;
use select::predicate::{Attr, Name};
use serde::Serialize;
use stdinix::stdinix;

/// A rightmove property page scraper.
#[derive(Parser, Debug)]
#[command(name = "rightscrapex", group = ArgGroup::new("emit"))]
struct Opt {
    /// Activate filter on listings with floorplan tab
    #[arg(short, long)]
    floorplan: bool,

    /// Emit jsonlines
    #[arg(short, long, group = "emit")]
    json: bool,

    /// Emit urls
    #[arg(short, long, group = "emit")]
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

fn scrape(url: String, doc: Document) -> eyre::Result<ScrapeResult> {
    let summary = doc
        .find(Attr("itemprop", "streetAddress"))
        .next()
        .context(format!("Missing summary for {}", &url))?
        .text()
        .trim()
        .to_owned();
    let floorplan_url = doc
        .find(Attr("href", "#/floorplan?activePlan=1"))
        .next()
        .context(format!("Missing floorplans tab for {}", &url))
        .and_then(|anchor| {
            anchor
                .attr("href")
                .context(format!("Missing anchor for {}", &url))
                .and_then(|href| {
                    Ok(url.clone()
                        + &href[href
                            .bytes()
                            .position(|v| v == b'/')
                            .context("No / found in anchor")?
                            + 1..])
                })
        })
        .ok();

    let page_model = doc
        .find(Name("script"))
        .map(|node| node.text())
        .find(|text| text.contains("PAGE_MODEL"))
        .context("No page model found")?
        .split(" = ")
        .skip(1)
        .collect::<String>();

    // Data from page model.
    // Price: .propertyData.prices.primaryPrice
    // Human identifier: .propertyData.text.pageTitle
    // Location image URL: .propertyData.staticMapImgUrls.staticMapImgUrlMobile
    let model: serde_json::value::Value =
        serde_json::from_str(&page_model).context("Page model couldn't be parsed as json")?;
    let price = model
        .get("propertyData")
        .context("propertyData not found in model")?
        .get("prices")
        .context("prices not found in model")?
        .get("primaryPrice")
        .context("primaryPrice not found in prices")?
        .as_str()
        .context("primaryPrice wasn't a json string")?
        .to_owned();
    let human_identifier = model
        .get("propertyData")
        .context("propertyData not found in model")?
        .get("text")
        .context("text wasn't found model")?
        .get("pageTitle")
        .context("pageTitle wasn't found in text")?
        .as_str()
        .context("pageTitle wasn't a json-string")?
        .to_owned();
    let location_image_url = model
        .get("propertyData")
        .context("propertyData not found in model")?
        .get("staticMapImgUrls")
        .context("staticMapImgUrls wasn't found model")?
        .get("staticMapImgUrlMobile")
        .context("staticMapImgUrlMobile wasn't found in image urls")?
        .as_str()
        .context("staticMapImgUrlMobile wasn't a json-string")?
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

fn filter<'b>(cfg: &Opt, res: &'b ScrapeResult) -> Option<&'b ScrapeResult> {
    if cfg.floorplan && res.floorplan_url.is_none() {
        None
    } else {
        Some(res)
    }
}

fn main() -> eyre::Result<()> {
    let cfg = Opt::parse();

    stdinix(|line| {
        let body = ureq::get(line).call()?.body_mut().read_to_string()?;

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
                    println!(
                        "{}",
                        serde_json::to_string(&res).context("Failed to serialize json")?
                    );
                    std::io::stdout().flush().context("failed to flush stdout")
                }
                (false, true) => {
                    println!("{}", res.url);
                    std::io::stdout().flush().context("failed to flush stdout")
                }
                _ => panic!("No emit settings!"),
            }
        } else {
            Ok(())
        }
    })
}
