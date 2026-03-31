use image::RgbImage;
use imageproc::{
	drawing::{draw_filled_rect, flood_fill_mut},
	rect::{Rect, Region},
	utils::load_image_or_panic,
};
use rand::{Rng, rngs::ThreadRng};
use std::{collections::HashMap, env, hash::Hash, path::Path};

fn main() {
	let _args: Vec<String> = env::args().collect();
	let _options: Vec<(usize, (String, String))> = _args
		.iter()
		.enumerate()
		.take_while(|(_, i)| *i != "-")
		.filter_map(|(i, v)| match v {
			_ if v.starts_with("-") => Some((i, (v.into(), _args[i + 1].to_owned()))),
			_ => None,
		})
		.collect();
	let mut options: HashMap<String, String> = HashMap::new();
	_options
		.clone()
		.into_iter()
		.for_each(|(_, (k, v))| _ = options.insert(k, v));
	let args: Vec<&str> = _args
		.iter()
		.enumerate()
		.filter(|(i, _)| {
			_options
				.iter()
				.find(|(i1, _)| i1 == i || *i1 == i - 1)
				.is_none()
		})
		.map(|(_, s)| s.as_str())
		.collect();
	match &args[1..] {
		[source, destination] => {
			let score_done = options
				.get("-s")
				.and_then(|i| i.parse::<f64>().ok())
				.unwrap_or(1.0)
				.min(1.0);
			let save_iterations = options
				.get("--save-every-nth")
				.and_then(|i| i.parse::<usize>().ok())
				.unwrap_or(1);
			let o_continue = options
				.get("--continue")
				.and_then(|i| i.parse::<bool>().ok())
				.unwrap_or(true);

			let b: RgbImage = load_image_or_panic(source).into();
			assert!(score(&b, &b) == 1.0);
			eprintln!("source = {:?}", source);

			let destination = Path::new(destination);

			let mut a = RgbImage::new(b.width(), b.height());
			flood_fill_mut(&mut a, 0, 0, **find_mode(&b.pixels().collect::<Vec<_>>()));

			if o_continue {
				a = load_image_or_panic(destination).into();
			}

			let mut rng = rand::rng();

			let mut n = 1;
			let mut s = 0.0;
			for i in n.. {
				if s >= score_done {
					break;
				}

				let a1 = draw(&mut rng, &a, &b, s);
				let s1 = score(&a1, &b);

				if s1 <= s {
					continue;
				}

				(a, s, n) = (a1, s1, n + 1);

				if save_iterations > 0 && n % save_iterations == 0 {
					_ = a.save(destination);
				}

				eprintln!(
					"#{}+{} {:.12} / {} = {:.3}",
					n,
					i - n,
					s,
					score_done,
					s / score_done
				);
			}

			a.save(destination).unwrap();
			println!("{}", destination.display())
		}
		i => panic!(
			"invalid args: {:?}\n{}{}",
			i,
			format!("{:?} <source> <destination>", args[0]),
			[
				"-s 1.0 <0.0..=1.0>",
				"--save-every-nth 1 <usize>",
				"--continue true <bool>"
			]
			.iter()
			.map(|i| format!("\n\t{}", i))
			.collect::<Vec<_>>()
			.concat()
		),
	}
}

fn score(a: &RgbImage, b: &RgbImage) -> f64 {
	1.0 - a
		.pixels()
		.zip(b.pixels())
		.map(|(i0, i1)| {
			[(i0[0], i1[0]), (i0[1], i1[1]), (i0[2], i1[2])]
				.iter()
				.map(|&(a, b)| (a as f64 - b as f64).abs() / 255.0)
				.sum::<f64>()
				/ 3.0
		})
		.sum::<f64>()
		/ (a.width() * a.height()) as f64
}

fn draw(r: &mut ThreadRng, a: &RgbImage, b: &RgbImage, f: f64) -> RgbImage {
	let f = 1.0 - f / 1.5;
	let (w0, h0) = (a.width(), a.height());
	let s = w0.min(h0);
	let (w, h) = (
		1 + r.random_range(0..=(s as f64 * f) as u32),
		1 + r.random_range(0..=(s as f64 * f) as u32),
	);
	let rect = Rect::at(
		r.random_range(0..=w0 - w) as i32,
		r.random_range(0..=h0 - h) as i32,
	)
	.of_size(w, h);
	draw_filled_rect(
		a,
		rect,
		**find_mode(
			&b.enumerate_pixels()
				.filter(|&(x, y, _)| rect.contains(x as i32, y as i32))
				.map(|(_, _, v)| v)
				.collect::<Vec<_>>(),
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
