use gnuplot::{Figure, Caption, Color};

use super::genetic::Stats;

pub fn plot(stats: Stats) {
	let x: Vec<usize> = (0..(stats.average_scores.len())).collect();
	let y = stats.average_scores;
	let mut fg = Figure::new();
	fg.axes2d()
	.lines(&x, &y, &[Caption("A line"), Color("black")]);
	match fg.show() {
		Ok (_) => { println!("Plot success") },
		Err (err) => {println!("{:?}", err)}
	}
}
