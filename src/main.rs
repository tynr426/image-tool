
mod elapsed;
mod config;
mod compress;
use config::{Config};
use compress::CompressPicture;
use std::time::{Duration, Instant};
fn main() {
    
    
    use std::path::Path;
    //以当前目录为源目录，以当前目录+scale目录
    let src_path = Path::new(".");
    let source_path = src_path.join("source");
    let source_path = source_path.as_path();
    println!("ss{:?}",source_path);

    let cnf: Config = Config::new("");
    let com=CompressPicture::new(cnf);
    let souce_paths: Vec<&str> = vec!["./source/2_1024.jpg", "./source/3_1024.jpg", "./source/5_1024.jpg"];
    
    com.compress(&souce_paths);
}
