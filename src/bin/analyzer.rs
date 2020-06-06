use anyhow::{anyhow, Result};
use clap::Clap;

fn main() -> Result<()> {
    let opts = bright::Opts::parse();
    let files = std::fs::read_dir(opts.dir)?;
    for file in files {
        let file = file?;
        let img = image::open(file.path())?;
        match img {
            image::DynamicImage::ImageRgb8(img) => {
                let mut average = 0u64;
                for pixel in img.pixels() {
                    average += pixel[0] as u64;
                }
                average /= img.width() as u64 * img.height() as u64;
                println!("{}", average);
            }
            _ => {}
        }
    }

    Ok(())
}
