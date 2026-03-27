use image::RgbImage;
use imageproc::{
	drawing::draw_filled_rect,
	rect::{Rect, Region},
	utils::load_image_or_panic,
};
use rand::Rng;
use std::{collections::HashMap, env, hash::Hash, path::Path};

const SCORE_MIN_DELTA: f64 = 0.0;
const SCORE_DONE: f64 = 1.0;
const SAVE_ITERATIONS: bool = true;

fn main() {
	let _args: Vec<String> = env::args().collect();
	let args: Vec<&str> = _args.iter().map(|s| s.as_str()).collect();
	match &args[1..] {
		[source, destination] => {
			let destination = Path::new(destination);
			let b: RgbImage = load_image_or_panic(source).into();

			assert!(score(&b, &b) == 1.0);

			let mut a = RgbImage::new(b.width(), b.height());
			eprintln!("source = {:?}", source);

			let mut s = 0.0;
			for i in 0.. {
				let a1 = draw(a.clone(), &b);
				let s1 = score(&a1, &b);
				if s1 - s < SCORE_MIN_DELTA {
					continue;
				}
				a = a1;
				s = s1;
				eprintln!("#{} {:.12} / {} = {:.3}", i, s, SCORE_DONE, s / SCORE_DONE);
				if s >= SCORE_DONE {
					break;
				}
				if SAVE_ITERATIONS {
					a.save(destination).unwrap();
				}
			}

			a.save(destination).unwrap();
			println!("{}", destination.display())
		}
		i => panic!("invalid args: {:?}\n\t<source> <destination>", i),
	}
}

fn score(a: &RgbImage, b: &RgbImage) -> f64 {
	1.0 - a
		.pixels()
		.zip(b.pixels())
		.map(|(i0, i1)| {
			[(i0[0], i1[0]), (i0[1], i1[1]), (i0[2], i1[2])]
				.iter()
				.map(|(a, b)| (*a as f64 - *b as f64).abs() / 255.0)
				.sum::<f64>()
				/ 3.0
		})
		.sum::<f64>()
		/ (a.width() * a.height()) as f64
}

fn draw(a: RgbImage, b: &RgbImage) -> RgbImage {
	let mut r = rand::rng();
	let (w0, h0) = (a.width(), a.height());
	let (w, h) = (r.random_range(1..=w0), r.random_range(1..=h0));
	let rect = Rect::at(
		r.random_range(0..=w0 - w) as i32,
		r.random_range(0..=h0 - h) as i32,
	)
	.of_size(w, h);
	draw_filled_rect(
		&a,
		rect,
		**find_mode(
			b.enumerate_pixels()
				.filter(|(x, y, _)| rect.contains(*x as i32, *y as i32))
				.map(|(_, _, v)| v)
				.collect::<Vec<_>>()
				.as_slice(),
		),
	)
}

fn find_mode<T: Hash + std::cmp::Eq>(i: &[T]) -> &T {
	let mut o = HashMap::new();
	for e in i {
		*o.entry(e).or_insert(0) += 1;
	}
	o.into_iter().max_by_key(|&(_, i)| i).unwrap().0
}
