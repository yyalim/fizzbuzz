use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Play {
        #[clap(short, long)]
        count: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Play { count } => {
            if let Some(count) = count {
                let count = count.parse::<i32>().unwrap();
                fizzbuzz::play(count).iter().for_each(|s| println!("{}", s));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    #[test]
    fn test_play() {
        let mut cmd = Command::cargo_bin("fizzbuzz").unwrap();
        cmd.arg("play").arg("--count").arg("5");
        cmd.assert().success().stdout(contains("1\n2\nFizz\n4\nBuzz"));
    }
}
