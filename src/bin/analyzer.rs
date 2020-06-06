use anyhow::{anyhow, Result};
use clap::Clap;

fn main() -> Result<()> {
    let opts = bright::Opts::parse();
    let files = std::fs::read_dir(opts.dir)?;
    let mut out = csv::Writer::from_path(opts.out)?;
    for file in files {
        let file = file?;

        let path = file.path();
        let exposure = path
            .file_name()
            .ok_or(anyhow!("No file name"))?
            .to_str()
            .ok_or(anyhow!("could not convert to string"))?
            .split('.')
            .next()
            .ok_or(anyhow!("No extension"))?
            .split('-')
            .skip(1)
            .next()
            .ok_or(anyhow!("No exposure part in name"))?;

        let img = image::open(file.path())?;
        match img {
            image::DynamicImage::ImageRgb8(img) => {
                let mut average = 0u64;
                for pixel in img.pixels() {
                    average += pixel[0] as u64;
                }
                average /= img.width() as u64 * img.height() as u64;
                out.write_record(&[exposure, &format!("{}", average)])?;
            }
            _ => {}
        }
    }

    Ok(())
}
