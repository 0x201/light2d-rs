use clap::Parser;

#[derive(Clone)]
pub struct RenderConfig {
    pub output_name: String,
    pub sample_count: usize,
    pub width: u32,
    pub height: u32,
}

impl RenderConfig {
    const DEFAULT_WIDTH: u32 = 512;
    const DEFAULT_HEIGHT: u32 = 512;
    const DEFAULT_SAMPLE_COUNT: usize = 1024;

    pub fn new(width: u32, height: u32, sample_count: usize, output_name: String) -> Self {
        Self {
            width,
            height,
            sample_count,
            output_name,
        }
    }

    pub fn from_args() -> Self {
        let args = Args::parse();

        Self {
            width: args.width.unwrap_or(Self::DEFAULT_WIDTH),
            height: args.height.unwrap_or(Self::DEFAULT_HEIGHT),
            sample_count: args.samples.unwrap_or(Self::DEFAULT_SAMPLE_COUNT),
            output_name: args.output_filename,
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the output image
    #[clap(long, value_parser)]
    width: Option<u32>,

    /// Height of the output image
    #[clap(long, value_parser)]
    height: Option<u32>,

    /// Number of samples per pixel
    #[clap(long, value_parser)]
    samples: Option<usize>,

    /// Output filename
    #[clap(value_parser)]
    output_filename: String,
}
