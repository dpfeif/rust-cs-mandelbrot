use clap::{ArgEnum, Parser};
use mandelbrot::complex::*;
use mandelbrot::pixels::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the PNG file to create
    #[clap(default_value = "mandel.png")]
    file_name: String,

    /// Pixel size of the PNG file to create
    #[clap(default_value = "1000x750")]
    pixels: String,

    /// Complex point in the upper left corner of the frame
    #[clap(long, short, default_value = "-1.20,0.35")]
    upper_left: String,

    /// Complex point in the lower right corner of the frame
    #[clap(long, short, default_value = "-1,0.20")]
    lower_right: String,

    /// Parallelism choice
    #[clap(arg_enum, long, short, default_value_t=Parallelism::SingleThread)]
    parallelism: Parallelism,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Parallelism {
    SingleThread,
    CrossBeam,
    Rayon,
}

fn main() {
    let args = Args::parse();

    let bounds = parse_pair(&args.pixels, 'x').expect("error parsing image dimensions");
    let upper_left =
        parse_complex(&args.upper_left).expect("error parsing upper left corner point");
    let lower_right =
        parse_complex(&args.lower_right).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    match args.parallelism {
        Parallelism::SingleThread => render(&mut pixels, bounds, upper_left, lower_right),
        Parallelism::CrossBeam => {
            let threads = 8;
            let rows_per_band = bounds.1 / threads + 1;
            {
                let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
                crossbeam::scope(|spawner| {
                    for (i, band) in bands.into_iter().enumerate() {
                        let top = rows_per_band * i;
                        let height = band.len() / bounds.0;
                        let band_bounds = (bounds.0, height);
                        let band_upper_left =
                            pixel_to_point(bounds, (0, top), upper_left, lower_right);
                        let band_lower_right = pixel_to_point(
                            bounds,
                            (bounds.0, top + height),
                            upper_left,
                            lower_right,
                        );
                        spawner.spawn(move |_| {
                            render(band, band_bounds, band_upper_left, band_lower_right);
                        });
                    }
                })
                .unwrap();
            }
        }
        Parallelism::Rayon => rayon_render(&mut pixels, bounds, upper_left, lower_right),
    }

    write_image(&args.file_name, &pixels, bounds).expect("error writing PNG file");
}
