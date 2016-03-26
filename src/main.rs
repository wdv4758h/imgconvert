#[macro_use]
extern crate clap;
extern crate image;

use clap::App;

use std::fs::File;
use std::path::Path;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let infile = matches.value_of("IMAGE").unwrap();
    let outfile = matches.value_of("OUT").unwrap();
    let mut img = image::open(infile).unwrap();

    let size = matches.value_of("SIZE").unwrap_or("");
    if size.len() > 0 {
        let tmp = size.split('x').take(2).collect::<Vec<_>>();
        let (row, column) = (u32::from_str_radix(tmp[0], 10).unwrap(),
                             u32::from_str_radix(tmp[1], 10).unwrap());
        img = img.resize(row, column, image::FilterType::Gaussian);
    }

    img.save(&mut File::create(&Path::new(&outfile)).unwrap(),
             image::ImageFormat::JPEG).unwrap();
}
