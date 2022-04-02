use clap::Parser;
use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

#[derive(Parser)]
struct Cli {
    save: String,
    pattern: String,
}

error_chain! {
    foreign_links {
        Reqerror(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pb= indicatif::ProgressBar::new(100);
    let args = Cli::parse();

    let save_name = &args.save;

    let user_url = &args.pattern;

    let res = reqwest::get(user_url).await?.text().await?;

    let document = Document::from(res.as_str());

    let list_of_x: Vec<_> = document
        .find(Name("img"))
        .filter_map(|n| n.attr("src"))
        .into_iter()
        .collect();

    let path = save_name;
    let mut writerr = csv::Writer::from_path(path).unwrap();
    for row in list_of_x {
        writerr.write_record(&[row]).unwrap();
        writerr.flush()?;
        pb.println(format!("[+] finished #{}", row));
        pb.inc(1);
    }


    Ok(())
}
