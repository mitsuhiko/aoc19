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
                layer.iter().fold((0, 0, 0), |(z, o, t), x| match *x {
                    0 => (z + 1, o, t),
                    1 => (z, o + 1, t),
                    2 => (z, o, t + 1),
                    _ => (z, o, t),
                })
            })
            .min_by_key(|x| x.0)
            .map_or(0, |(_, o, t)| o * t)
    }

    fn merge_layers(&self) -> Vec<u8> {
        let mut rv = vec![2; self.width * self.height];
        for layer in self.layers.iter().rev() {
            for (idx, &pixel) in layer.iter().enumerate() {
                if pixel != 2 {
                    rv[idx] = pixel;
                }
            }
        }
        rv
    }

    fn draw(&self) -> String {
        self.merge_layers()
            .into_iter()
            .enumerate()
            .flat_map(|(idx, pixel)| {
                if idx > 0 && idx % self.width == 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(Some(match pixel {
                    0 => ' ',
                    1 => '*',
                    _ => '.',
                }))
            })
            .collect()
    }
}

fn main() {
    let image = Image::decode(include_str!("../input.txt"), 25, 6);
    println!("part 1: {}", image.find_best_layer_sum());
    println!("part 2:");
    println!();
    println!("{}", image.draw());
}
