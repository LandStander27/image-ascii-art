use std::io::Write;
use image::GenericImageView;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

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

	fn _get_ascii(&self, intensity: u32) -> String {
		let i = intensity/32;
		return [" ", ".", ",", "-", "~", "+", "=", "@"][i as usize].to_string();
	}

}

fn read() -> String {
	let mut buffer: String = String::new();
	let stdin: std::io::Stdin = std::io::stdin();
	stdin.read_line(&mut buffer).unwrap();
	return buffer.trim().to_string();
}

fn main() {

	println!("Loading.");
	print!("Path to image ? ");
	std::io::stdout().flush().unwrap();
	let path = read();
	if !std::path::Path::new(path.as_str()).exists() {
		eprintln!("Path does not exist");
		rpassword::prompt_password("Press enter to exit ").unwrap();
	}
	let mut img = AsciiImage::new(path.as_str());
	print!("Ratio of pixels to characters ? ");
	std::io::stdout().flush().unwrap();
	let ratio = read();
	img.create(ratio.parse().unwrap_or(1));

	let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
	ctx.set_contents(img.ascii_string).unwrap();
	println!("Copied to clipboard.");
	rpassword::prompt_password("Press enter to exit ").unwrap();

}