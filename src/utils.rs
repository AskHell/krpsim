use std::fmt::Debug;

pub fn generalize_error<T: Debug>(err: T) -> String {
	format!("{:?}", err)
}

pub fn fibonacci_n(n: usize) -> Vec<usize> {
    let (list, _) = (1..100).fold((vec![], 0), |(mut acc, total), i| {
        if total > n {
            return (acc, total)
        }
        let to_append = if i < 3 {
			1
		} else {
		    let a = acc.get(i - 3).unwrap_or(&0);
			let b = acc.get(i - 2).unwrap_or(&1);
			a + b
		};
		let new_total = total + to_append;
		if new_total > n {
			let mut padding = vec![1; n - total];
			padding.append(&mut acc);
			return (padding, new_total)
		}
        acc.push(to_append);
        (acc, new_total)
    });
	list
}
