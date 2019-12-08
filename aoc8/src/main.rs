use itertools::Itertools;

#[derive(Debug)]
struct Image {
    width: usize,
    height: usize,
    layers: Vec<Vec<u8>>,
}

impl Image {
    fn decode(data: &str, width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            layers: data
                .trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .chunks(width * height)
                .into_iter()
                .map(|x| x.collect())
                .collect(),
        }
    }

    fn find_best_layer_sum(&self) -> usize {
        self.layers
            .iter()
            .map(|layer| {
                layer.iter().fold((0, 0, 0), |(z, o, t), p| match *p {
                    0 => (z + 1, o, t),
                    1 => (z, o + 1, t),
                    2 => (z, o, t + 1),
                    _ => unreachable!(),
                })
            })
            .min_by_key(|x| x.0)
            .map_or(0, |(_, o, t)| o * t)
    }

    fn draw(&self) -> String {
        self.layers
            .iter()
            .rev()
            .fold(vec![0; self.width * self.height], |out, layer| {
                layer
                    .iter()
                    .enumerate()
                    .fold(out, |mut out, (idx, &pixel)| {
                        if pixel != 2 {
                            out[idx] = pixel;
                        }
                        out
                    })
            })
            .into_iter()
            .enumerate()
            .fold(String::new(), |mut out, (idx, pixel)| {
                if idx > 0 && idx % self.width == 0 {
                    out.push('\n');
                }
                out.push(if pixel == 1 { '*' } else { ' ' });
                out
            })
    }
}

fn main() {
    let image = Image::decode(include_str!("../input.txt"), 25, 6);
    println!("part 1: {}", image.find_best_layer_sum());
    println!("part 2:");
    println!();
    println!("{}", image.draw());
}
