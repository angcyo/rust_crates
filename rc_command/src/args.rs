///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/27
///
use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    pub count: u8,
}

#[cfg(test)]
mod tests {
    use crate::args::Args;
    use clap::Parser;

    #[test]
    fn test() {
        let args = Args::parse();

        for _ in 0..args.count {
            println!("Hello {}!", args.name);
        }
    }
}
