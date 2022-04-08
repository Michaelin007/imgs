use anyhow::Context;
use clap::Parser;
use select::document::Document;
use select::predicate::Name;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    output: std::path::PathBuf,
    url: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let res =
        reqwest::blocking::get(&args.url).with_context(|| format!("opening url {:?}", args.url))?;

    let document = Document::from_read(res).context("parsing response")?;

    let writer = std::fs::File::create(&args.output)
        .with_context(|| format!("opening file {:?}", args.output))?;
    let mut writer = std::io::BufWriter::new(writer);

    for img_src in document.find(Name("img")).filter_map(|n| n.attr("src")) {
        writeln!(writer, "{img_src}").context("writing to the output file")?;
    }
    Ok(())
}
