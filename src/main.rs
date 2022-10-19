use asciifire::{AsciiImage, ResizeArgs};
use clap::Parser;
use anyhow::Result;



/// Program to convert and display an image as ascii text
#[derive(Parser, Debug)]
struct Args {
    /// Path to image file 
    path: std::path::PathBuf,
    /// # of lines of text (maintains aspect ratio unless width is also set)
    #[arg(long)]
    height: Option<u32>,
    /// # of characters on line / 2 (maintains aspect ratio unless height is also set)  
    #[arg(long)]
    width: Option<u32>,
    /// output result to file
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,
    /// filter type to resize by
    #[arg(short, long)]
    // filter: Option<image::imageops::FilterType>, 
    filter: Option<asciifire::Filter>, 
}

fn main() -> Result<()> {
    let args = Args::parse();
    // define desired behavior for CLI args
    let resize_args = match (&args.width, &args.height, &args.output) {
        (None, None, None) => ResizeArgs::Terminal,
        (None, None, Some(_)) => ResizeArgs::Original,
        (&Some(w), None, _) => ResizeArgs::Width(w),
        (None, &Some(h), _) => ResizeArgs::Height(h),
        (&Some(w), &Some(h), _) => ResizeArgs::WidthAndHeight(w, h),
    };
    let ascii = AsciiImage::from_path(&args.path, &args.filter, resize_args)?;
    if let Some(path) = &args.output {
        ascii.write_to_file(&path)?;
    } else {
        println!("{}", ascii);
    };
    Ok(())
}
