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
    CrossBeam,
    RayonRow,
    RayonPixel,
    SingleThread,
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
        Parallelism::CrossBeam => crossbeam_render(&mut pixels, bounds, upper_left, lower_right),
        Parallelism::RayonRow => rayon_row_render(&mut pixels, bounds, upper_left, lower_right),
        Parallelism::RayonPixel => rayon_pixel_render(&mut pixels, bounds, upper_left, lower_right),
        Parallelism::SingleThread => render(&mut pixels, bounds, upper_left, lower_right),
    }

    write_image(&args.file_name, &pixels, bounds).expect("error writing PNG file");
}
