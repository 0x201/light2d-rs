use clap::Parser;

#[derive(Clone)]
pub struct RenderConfig {
    pub output_name: String,
    pub output_dir: String,
    pub sample_count: usize,
    pub width: u32,
    pub height: u32,
    pub scene: String,
    pub scene_file: Option<String>,
}

impl RenderConfig {
    const DEFAULT_WIDTH: u32 = 512;
    const DEFAULT_HEIGHT: u32 = 512;
    const DEFAULT_SAMPLE_COUNT: usize = 1024;
    const DEFAULT_OUTPUT_DIR: &'static str = "output";

    pub fn from_args() -> Self {
        let args = Args::parse();

        Self {
            width: args.width.unwrap_or(Self::DEFAULT_WIDTH),
            height: args.height.unwrap_or(Self::DEFAULT_HEIGHT),
            sample_count: args.samples.unwrap_or(Self::DEFAULT_SAMPLE_COUNT),
            output_name: args.output_name,
            output_dir: args
                .output_dir
                .unwrap_or_else(|| Self::DEFAULT_OUTPUT_DIR.to_string()),
            scene: args.scene,
            scene_file: args.scene_file,
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
    output_name: String,

    /// Output directory
    #[clap(long, value_parser, default_value = "output")]
    output_dir: Option<String>,

    /// Scene name: shape, reflect, refaction, beer
    #[clap(long, value_parser, default_value = "beer")]
    scene: String,

    /// Path to a JSON scene description file (overrides --scene)
    #[clap(long, value_parser)]
    scene_file: Option<String>,
}
