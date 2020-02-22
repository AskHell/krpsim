use gnuplot::{Figure, Caption, Color};

use super::solver::Stats;

pub fn plot(stats: Stats) {
	let x: Vec<usize> = (0..(stats.average_scores.len())).collect();
	// let x = [0u32, 1, 2];
	// let y = [3u32, 4, 5];
	println!("scores {:?}", stats.average_scores);
	let y = stats.average_scores;
	let mut fg = Figure::new();
	fg.axes2d()
	.lines(&x, &y, &[Caption("A line"), Color("black")]);
	match fg.show() {
		Ok (_) => { println!("yeah") },
		Err (err) => {println!("{:?}", err)}
	}
}
