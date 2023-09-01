use clap::Parser;
use rand::distributions::{Distribution, Uniform};
use linked_lists::cs120::List;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of random operations to perform
    #[arg(short, long, default_value_t = 1000)]
    num_ops: u32,

    /// Range of random numbers to insert/delete from list 
    #[arg(short, long, default_value_t = 10)]
    value_range: i32,

    /// Print every print_freq operations 
    #[arg(short, long, default_value_t = 100)]
    print_freq: u32,
}

fn main() {
    // Process commandline arguments
    let args = Args::parse();
    let num_ops = args.num_ops;
    let value_range = args.value_range;
    let print_freq = args.print_freq;

    // Initialization
    let mut list = List::new();
    let dist = Uniform::new_inclusive(1,100);
    let mut rng = rand::thread_rng();

    for op in 1..num_ops+1 {
        let op_type = dist.sample(&mut rng) % 2;
        let value = dist.sample(&mut rng) % value_range + 1;

        if op_type == 0 {
            list.insert(value);
        } else {
            list.delete(value);
        }

        if op % print_freq == 0 {
            list.print();
            println!();
        }
    }
}
