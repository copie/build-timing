use build_timing::build_timing;

build_timing!(build);

fn main() {
    println!("{}", build::BUILD_OS);
    println!("Hello, world!");
    println!("{}", build::OKK);
    println!("{}", build::NONONO);
}
