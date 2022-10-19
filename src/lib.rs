use std::io::Write;

use image::{GenericImageView, DynamicImage};
use anyhow::{Context, Result};
use image::imageops::FilterType;
use termion::terminal_size;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Filter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl Filter {
    pub fn to_filter_type(&self) -> FilterType {
        match &self {
            Filter::Nearest => FilterType::Nearest,
            Filter::Triangle => FilterType::Triangle,
            Filter::CatmullRom => FilterType::CatmullRom,
            Filter::Gaussian => FilterType::Gaussian,
            Filter::Lanczos3 => FilterType::Lanczos3,
        }
    }
}

pub enum ResizeArgs {
    Original,
    TrueOriginal,
    Width(u32),
    Height(u32),
    WidthAndHeight(u32, u32),
    Terminal,
}

#[derive(Debug)]
pub struct AsciiImage {
    image: Vec<String>,
}

impl AsciiImage {
    pub fn new(image: Vec<String>) -> Self {
        AsciiImage { image } 
    }

    pub fn write_to_file(&self, path: &std::path::PathBuf) -> Result<()> {
        let f = std::fs::File::create(path)?;
        let mut f = std::io::BufWriter::new(f);
        for i in &self.image {
            f.write(format!("{}\n", i).as_bytes()).with_context(|| "Failed to write to file")?;
        }
        f.flush()?;
        Ok(())
    }

    pub fn to_ascii(img: &image::DynamicImage) -> Result<Self> {
        // faster?: converts to grayscale here and maps to char
        Ok(
            Self::new(
                img.pixels().collect::<Vec<(u32, u32, image::Rgba<u8>)>>().chunks(img.width() as usize).fold(Vec::with_capacity(img.height() as usize), |mut v, row| {
                    v.push(row.iter().fold(String::with_capacity(img.width() as usize), |mut s, &(.., p)| {
                        s.push(Self::map_char(to_grayscale(&p.0).into()));
                        s
                    }));
                    v
                })
            )
        )
    }

    pub fn from_path(path: &std::path::PathBuf, filter: &Option<Filter>, resize_args: ResizeArgs) -> Result<AsciiImage> {
        let img = open_image(&path)?;
        let ascii = Self::from_image(&img, filter, resize_args)?;
        Ok(ascii)
    }

    pub fn from_image(img: &image::DynamicImage, filter: &Option<Filter>, resize_args: ResizeArgs) -> Result<AsciiImage> {
        let filter_type = filter.unwrap_or(Filter::Triangle).to_filter_type();
        let img = match resize_args {
            ResizeArgs::TrueOriginal => img.clone(), // do not resize
            ResizeArgs::Original => img.resize_exact(img.width() * 2, img.height(), filter_type),
            ResizeArgs::Width(w) => {
                let h = ((w as f64 / img.width() as f64) * img.height() as f64) as u32;
                img.resize_exact(w, h / 2, filter_type)
            },
            ResizeArgs::Height(h) => {
                let w = ((h as f64 / img.height() as f64) * img.width() as f64) as u32;
                img.resize_exact(w * 2, h, filter_type)
            },
            ResizeArgs::WidthAndHeight(w, h) => {
                img.resize_exact(w, h, filter_type)
            },
            ResizeArgs::Terminal => {
                // maintain aspect ratio manually but twice as wide since terminal is 1x2
                if atty::is(atty::Stream::Stdout) {
                    let (tw, th) = terminal_size()?;
                    let mut w = tw as u32;
                    let mut h = th as u32;
                    if w >= h {
                        w = ((h as f64 / img.height() as f64) * img.width() as f64) as u32;
                        if (tw as u32) < w * 2 {
                            w = tw as u32 / 2;
                            h = ((w as f64 / img.width() as f64) * img.height() as f64) as u32;
                        } 
                    } else {
                        h = ((w as f64 / img.width() as f64) * img.height() as f64) as u32;
                        if (th as u32) < h * 2 {
                            h = th as u32 / 2;
                            w = ((h as f64 / img.height() as f64) * img.width() as f64) as u32;
                        }
                    }
                    img.resize_exact(w * 2, h.into(), filter_type)
                } else {
                    img.clone()
                }
            },
        };
        AsciiImage::to_ascii(&img)
    }

    const BRIGHTNESS: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "; 
    pub fn map_char(pixel: usize) -> char {
        Self::BRIGHTNESS.chars().nth(pixel * (Self::BRIGHTNESS.len() - 1) / u8::MAX as usize).unwrap()
    }
}


impl std::fmt::Display for AsciiImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.image {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}


fn open_image(path: &std::path::PathBuf) -> Result<DynamicImage> {
    let img = image::open(path).with_context(|| format!("Failed to open image at provided path: {:?}", path))?;
    Ok(img)
}

fn to_grayscale(&[r, g, b, a]: &[u8; 4]) -> u8 {
    let color: f64 = (0.3 * r as f64) + (0.59 * g as f64) + (0.11 * b as f64); 
    (color + (1. - (a as f64) / 255.) * (255. - color)) as u8
}
