use std::env::Args;

pub fn parse_arguments(mut args: Args) -> Vec<u8> {
    // Discard the first argument, which is just the executable path.
    args.next();

    let mut days = Vec::new();
    for arg in args {
        let arg = arg.parse::<u8>().expect("Error parsing argument");
        assert!(arg <= 25, "Provided puzzle number greater than 25");
        days.push(arg);
    }

    days
}
