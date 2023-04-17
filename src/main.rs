use image::GenericImageView;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::exit;

struct AsciiImage {
	ascii: Vec<Vec<String>>,
	ascii_string: String,
	image_path: String
}

impl AsciiImage {
	fn new(path: &str) -> Self {
		
		return Self {
			ascii: Vec::new(),
			ascii_string: String::new(),
			image_path: path.to_string(),
		};

	}

	fn create(&mut self, scale: u32) {
		let img = image::open(self.image_path.clone()).unwrap();

		let (width, height) = img.dimensions();

		for y in 0..height {
			if y % (scale*2) == 0 {
				let mut row: Vec<String> = Vec::new();
				for x in 0..width {
					if x % scale == 0 {
						let pixel = img.get_pixel(x, y);
						let mut intensity: u32 = pixel[0] as u32/3 + pixel[1] as u32/3 + pixel[2] as u32/3;
						if pixel[3] == 0 {
							intensity = 0;
						}

	
						row.push(self._get_ascii(intensity));
					}
				}
				self.ascii.push(row.clone());
				self.ascii_string.push_str(&row.join(&"".to_string()));
				self.ascii_string.push_str("\n");
			}

		}

	}

	fn save_to_file(&self, path: &str) {
		if Path::new(path).exists() {
			std::fs::remove_file(path.clone()).unwrap();
		}
		let mut file = File::create(path).unwrap();
		file.write_all(self.ascii_string.as_bytes()).unwrap();
	}

	fn _get_ascii(&self, intensity: u32) -> String {
		let i = intensity/32;
		return [" ", ".", ",", "-", "~", "+", "=", "@"][i as usize].to_string();
	}

}

fn main() {

	let help_string: &str = "\nUsage: 
image_ascii.exe [image] {options}

Options:

--help:        Show this menu.
--output:      Outputs ascii art to text file (ascii.txt).
--ratio [int]: The ratio of pixels to ascii (higher the number the smaller the resulting image) (default: 3).";

	let args: Vec<String> = std::env::args().collect();

	if args.len() == 1 {
		println!("{}", help_string);
		exit(0);
	}
	if args[1..].contains(&"--help".to_string()) {
		println!("{}", help_string);
		exit(0);
	}

	let index = args.clone().iter().position(|x| { x == "--ratio" }).unwrap_or(0);
	let ratio = if index == 0 {
		3
	} else {
		if args.len() <= index+1 {
			eprintln!("Must supply ratio after --ratio");
			exit(1);
		}
		match args[index+1].parse::<i32>() {
			Ok(val) => if val <= 0 {
				eprintln!("Ratio must be more than 0");
				exit(1);
			} else {
				val
			},
			Err(_) => {
				eprintln!("Must supply ratio after --ratio");
				exit(1);
			}
		}
	};

	let path = (&args[1]).clone();
	if !std::path::Path::new(path.as_str()).exists() {
		eprintln!("Path does not exist");
		exit(1);
	}
	println!("Creating.");
	let mut img = AsciiImage::new(path.as_str());
	img.create(ratio as u32);

	if args[1..].contains(&"--output".to_string()) {
		img.save_to_file(".\\ascii.txt");
		println!("Saved to .\\ascii.txt");
	} else {
		let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
		ctx.set_contents(img.ascii_string).unwrap();
		println!("Copied to clipboard.");
	}

}