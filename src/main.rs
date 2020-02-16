use std::io::{Result, Write};

use oops::Oops;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serde::Serialize;
use stdinix::stdinix;
use structopt::{clap::ArgGroup, StructOpt};
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
        .find(Class("fs-22"))
        .next()
        .oops("Missing summary")?
        .text()
        .trim()
        .to_owned();
    let human_identifier = doc
        .find(Class("fs-16"))
        .next()
        .oops("Missing subtitle")?
        .text()
        .trim()
        .to_owned();
    let price = doc
        .find(Attr("id", "propertyHeaderPrice").descendant(Name("strong")))
        .next()
        .oops("Missing price")?
        .text()
        .trim()
        .to_owned();
    let floorplan_url = doc
        .find(Attr("id", "floorplansTab"))
        .next()
        .oops("Missing floorplans tab")
        .and_then(|tab| {
            tab.find(Name("a"))
                .next()
                .oops("Missing anchor")
                .and_then(|node| {
                    node.attr("href")
                        .oops("Missing href")
                        .map(|h| url.clone() + h)
                })
        })
        .ok();
    let location_image_url = doc
        .find(Class("js-ga-minimap").descendant(Name("img")))
        .next()
        .oops("Missing minimap")?
        .attr("src")
        .oops("Missing src")?
        .to_owned()
        .trim()
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
        let body = ureq::get(&line[..]).call().into_string()?;

        let doc = Document::from(&body[..]);

        let res = scrape(line.trim().to_owned(), doc)?;

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
