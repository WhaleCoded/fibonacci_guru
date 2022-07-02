use clap::{self, clap_app};

mod dynamic;
mod recursive;

#[tokio::main]
async fn main() {
    let commandline_arg_matches = clap::clap_app!(test_cli =>
        (about: "Starts an instance of the agent service")
        (version: "1.0")
        (@arg DYNAMIC: -d --dynamic + takes_value "Uses the dynamic programming implementation")
        (@arg RECURSIVE: -r --recursive + takes_value "Uses the recursive implementation")
        (@arg BOTH: -b --both + takes_value "Uses both the dynamic programming and the recursive implementation.")
        (@arg LIMIT: -l --limit + takes_value "Changes to the recursive implementation")
    )
    .get_matches();
}
