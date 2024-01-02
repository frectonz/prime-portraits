use clap::Parser;
use color_eyre::{eyre::Context, Result, Section};
use image::{
    imageops::{dither, ColorMap},
    io::Reader as ImageReader,
    Rgb,
};
use num_bigint::BigUint;
use rand::Rng;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image file
    file: String,

    /// The width of the prime number image generated.
    #[arg(long, default_value_t = 30)]
    width: u32,

    /// The height of the prime number image generated.
    #[arg(long, default_value_t = 60)]
    height: u32,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let Args {
        file,
        width,
        height,
    } = Args::parse();

    let mut img = ImageReader::open(&file)
        .wrap_err(format!("Unable to read image: '{}'", &file))
        .suggestion("try using a file that exists next time")?
        .decode()
        .wrap_err(format!("Unable to decode image: '{}'", &file))
        .suggestion(format!("are you sure '{}' is a valid image file", file))?
        .thumbnail(width, height)
        .into_rgb8();

    dither(&mut img, &MyColorMap);

    let (width, height) = (img.width(), img.height());

    let mut digits = img
        .pixels()
        .map(|x| (x.0[0] % 9) as u32)
        .collect::<Vec<_>>();

    println!("I have converted the image into a number");
    print_big_num(&digits, width, height);

    println!("I am now calculating the prime number version, this will take a long time");
    // let img_num = next_prime(&digits);
    // print_big_num(&img_num, width, height);

    let mut rng = rand::thread_rng();
    loop {
        let positions = (0..1)
            .map(|_| rng.gen_range(1..digits.len()))
            .collect::<Vec<_>>();

        let originals = positions
            .into_iter()
            .map(|pos| {
                let elem = digits.get_mut(pos).unwrap();
                let original = *elem;
                *elem = rng.gen_range(0..=9);
                (pos, original)
            })
            .collect::<Vec<_>>();

        if is_prime(&digits) {
            break;
        } else {
            originals.into_iter().for_each(|(pos, original)| {
                let elem = digits.get_mut(pos).unwrap();
                *elem = original;
            })
        }
    }
    print_big_num(&digits, width, height);

    Ok(())
}

fn _next_prime(n: &Vec<u32>) -> Vec<u32> {
    let n = num_bigint_dig::prime::next_prime(&num_bigint_dig::BigUint::from_bytes_le(
        &BigUint::new(n.to_owned()).to_bytes_le(),
    ));
    BigUint::from_bytes_le(&n.to_bytes_le()).to_u32_digits()
}

fn is_prime(n: &Vec<u32>) -> bool {
    num_bigint_dig::prime::probably_prime(
        &num_bigint_dig::BigUint::from_bytes_le(&BigUint::new(n.to_owned()).to_bytes_le()),
        2,
    )
}

fn print_big_num(digits: &[u32], width: u32, height: u32) {
    for y in 0..width {
        for x in 0..height {
            match digits.get((y * width + x) as usize) {
                Some(x) => print!("{x}"),
                None => print!(" "),
            }
        }
        println!();
    }

    println!(
        "num = {}",
        digits
            .into_iter()
            .map(|d| d.to_string())
            .fold(String::new(), |mut acc, d| {
                acc.push_str(&d);
                acc
            })
    )
}

struct MyColorMap;

impl ColorMap for MyColorMap {
    type Color = Rgb<u8>;

    fn index_of(&self, _: &Self::Color) -> usize {
        unimplemented!()
    }

    fn map_color(&self, color: &mut Self::Color) {
        let [r, g, b] = color.0;
        let grayscale = ((r as u32 + g as u32 + b as u32) / 3) as u8;
        color.0 = [grayscale; 3];
    }
}
