use std::process;

use image::{imageops, GenericImageView};

use super::cli;

/* Characters will be used depending on pixel's approximate grayscale value.
** Values are organized from darkest to lightest.
*/
const CHARACTERS: [char; 9] = ['@', '#', '8', '&', 'o', ':', '*', '.', ' '];

// default number of characters for width
const DEFAULT_WIDTH: u32 = 120;

// to respect height/width ratio of characters. The multiplier value is selected through trial and error.
const MULTIPLIER: f64 = 0.45;

fn to_grayscale(file: &str) -> image::DynamicImage {
    let img = image::open(file).unwrap_or_else(|_| {
        eprintln!("The file: {} you have provided is not found.", file);
        process::exit(2);
    });
    let img = img.grayscale();
    img
}

fn resize(img: image::DynamicImage, width: u32) -> image::DynamicImage {
    let (x, y) = img.dimensions();
    let height = (width * y / x) as f64 * MULTIPLIER;
    let img = img.resize_exact(width, height.floor() as u32, imageops::FilterType::Triangle);
    img
}

fn replace_pixel_with_char(img: image::DynamicImage) -> Vec<Vec<char>> {
    let mut chars = vec![];
    let (x, y) = img.dimensions();
    for j in 0..y {
        let mut row = vec![];
        for i in 0..x {
            let pixel = img.get_pixel(i, j);
            let c = match pixel.0[0] {
                // any of the rgb values is fine because they are the same for greyscale
                0..=28 => CHARACTERS[0],
                29..=56 => CHARACTERS[1],
                57..=84 => CHARACTERS[2],
                85..=112 => CHARACTERS[3],
                113..=140 => CHARACTERS[4],
                141..=168 => CHARACTERS[5],
                169..=196 => CHARACTERS[6],
                197..=224 => CHARACTERS[7],
                225..=255 => CHARACTERS[8],
            };
            row.push(c);
        }
        chars.push(row);
    }
    return chars;
}

fn print_chars(chars: Vec<Vec<char>>) {
    for rows in chars {
        for c in rows {
            print!("{}", c);
        }
        println!();
    }
}

fn print_files(args: &cli::Cli) {
    for file in &args.files {
        let img = to_grayscale(file);

        let size = if let Some(s) = &args.size {
            match s {
                cli::Size::ExtraSmall => 30,
                cli::Size::Small => 60,
                cli::Size::Large => 240,
                cli::Size::ExtraLarge => 360,
                _ => DEFAULT_WIDTH,
            }
        } else {
            DEFAULT_WIDTH
        };

        let img = resize(img, size);

        if args.files.len() > 1 {
            let mut path = file.split('.').collect::<Vec<&str>>();
            let name = if path.len() == 1 {
                path[0]
            } else {
                path.reverse();
                path[1]
            };
            println!("{}:", name);
        }

        let img_chars = replace_pixel_with_char(img);
        print_chars(img_chars);
    }
}

pub fn run(args: cli::Cli) {
    if args.files.len() > 0 {
        print_files(&args);
    }
}
