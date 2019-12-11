use std::{env, fmt, io};

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let width: u32 = args.next().unwrap().parse().unwrap();
    let height: u32 = args.next().unwrap().parse().unwrap();

    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().to_string();

    let image = Image::new(width, height, &input);
    println!("Part 1: {}", find_fewest_zeros(&image));
    println!("Part 2: \n{}", image);
}

fn find_fewest_zeros(image: &Image) -> u32 {
    let mut min = None;
    let mut min_layer = None;
    for (i, layer) in image.layers.iter().enumerate() {
        let num_zeros = layer
            .iter()
            .fold(0, |acc, x| if *x == 0 { acc + 1 } else { acc });
        if let Some(current_min) = min {
            if num_zeros < current_min {
                min = Some(num_zeros);
                min_layer = Some(i);
            }
        } else {
            min = Some(num_zeros);
            min_layer = Some(i);
        }
    }

    let mut ones = 0;
    let mut twos = 0;
    for x in image.layers[min_layer.unwrap()].iter() {
        if *x == 1 {
            ones += 1;
        } else if *x == 2 {
            twos += 1;
        }
    }

    ones * twos
}

struct Image {
    width: u32,
    height: u32,
    layers: Vec<Vec<u32>>,
}

impl Image {
    fn new(width: u32, height: u32, image_data: &str) -> Image {
        let mut layers = Vec::new();
        let mut layer = Vec::new();

        let layer_size = (width * height) as usize;

        let chars = image_data.chars().filter(|c| !c.is_whitespace());

        for (i, c) in chars.enumerate() {
            if i > 0 && i % layer_size == 0 {
                layers.push(layer);
                layer = Vec::new();
            }

            let x: u32 = c.to_digit(10).unwrap();
            layer.push(x);
        }

        layers.push(layer);

        Image {
            width,
            height,
            layers,
        }
    }

    fn decode(&self) -> String {
        let mut image = vec![2; (self.width * self.height) as usize];

        for i in 0..image.len() {
            for layer in self.layers.iter() {
                if i < layer.len() && layer[i] != 2 {
                    image[i] = layer[i];
                    break;
                }
            }
        }

        image.into_iter().map(|x| x.to_string()).collect()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decoded = self.decode();
        let mut lines = Vec::new();
        let mut s = &decoded[..];
        while s.len() >= self.width as usize {
            let (left, right) = s.split_at(self.width as usize);
            lines.push(left);
            s = right;
        }
        let out: String = lines.join("\n").replace("0", " ").replace("1", "█");

        write!(f, "{}", out)
    }
}

#[test]
fn example_works() {
    let image = Image::new(2, 2, "0222112222120000");
    assert_eq!(&image.decode(), "0110");

    let image = Image::new(2, 2, "2222 2122 00  ");
    assert_eq!(&image.decode(), "0122");

    let image = Image::new(2, 2, "222221210010");
    assert_eq!(&image.decode(), "0111");

    let image = Image::new(
        3,
        3,
        "
		122
		201
		211

		002
		111
		222

		000
		000
		000
		",
    );
    assert_eq!(&image.decode(), "100101011");
}
