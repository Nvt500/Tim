use std::env;
use std::process;


fn main() 
{
    let args: Vec<String> = env::args().collect();

    let config = tim::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}.");
        process::exit(1);
    });

    if let Err(err) = tim::run(config)
    {
        eprintln!("{err}");
        process::exit(1);
    }
}