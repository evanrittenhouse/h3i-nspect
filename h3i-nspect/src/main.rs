mod sections;
mod test_case;
mod util;

use clap::Parser;
use sections::generate_sections;
use util::Host;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let sections = generate_sections(args.host.0);

    // TODO: run all of these on a separate joinset (can probably optimize task run layout)
    for s in sections {
        s.run().await;
    }

    // enable testing full sections, and subsections too
}

#[derive(Parser, Debug)]
#[command(about = "HTTP/3 conformance test suite")]
struct Args {
    /// Host name for server under test.
    host: Host,
    // RFC-specified strictness for tests. Must be one of "must", "should", or "may". If no
    // strictness is specified, "must" will be used.
    // #[arg(long, value_name = "STRICTNESS")]
    // strictness: Option<Strictness>,
}
