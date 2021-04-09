use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    println!("mow-grep");
    let args = CliArgs::from_args();
    match std::fs::read_to_string(&args.path) {
        Ok(content) => {
            content.lines().for_each(|line| {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            });
        }
        Err(err) => {
            println!("Oh noes: {}", err)
        }
    }
}
