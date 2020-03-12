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

fn unite<T>(mut a: Vec<T>, mut b: Vec<T>) -> Vec<T> where T: Ord {
    let mut finish: Vec<T> = Vec::<T>::with_capacity(a.len() + b.len());

    while a.len() > 0 && b.len() > 0 {
        if a[0] > b[0] {
            finish.push(b.remove(0));
        } else {
            finish.push(a.remove(0));
        }
    }

    if a.len() > 0 {
        for e in a.into_iter() {
            finish.push(e);
        }
    } else {
        for e in b.into_iter() {
            finish.push(e);
        }
    }

    finish
}

pub fn merge_sort<T>(mut vec: Vec<T>) -> Vec<T> where T: Ord {
    let len = vec.len();

    if len <= 1 { vec }
    else {
        let b = vec.split_off(len / 2);
        let a = vec;

        let a = merge_sort(a);
        let b = merge_sort(b);

        unite(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;
    use rand::{
        seq::SliceRandom,
        thread_rng,
    };

    #[test]
    fn test_basic_merge_sort() {
        let i1_vec = vec![1, 2, 3, 4, 5, 6];
        let i2_vec = vec![6, 5, 4, 3, 2, 1];
        let i3_vec = vec![5, 6, 3, 2, 1, 4];
        let i4_vec = vec![2, 3, 4, 5, 6, 1];

        assert_eq!(merge_sort(i1_vec.clone()), i1_vec);
        assert_eq!(merge_sort(i2_vec), i1_vec);
        assert_eq!(merge_sort(i3_vec), i1_vec);
        assert_eq!(merge_sort(i4_vec), i1_vec);
    }

    #[test]
    fn test_big_merge_sort() {
        let mut rng = thread_rng();

        let truth: Vec<i32> = (0..10000).collect();
        let mut i1 = truth.clone();

        i1.shuffle(&mut rng);

        assert_eq!(merge_sort(i1), truth);
    }

    #[test]
    fn test_edge_merge_sort() {
        let v1: Vec<i32> = vec![];

        assert_eq!(merge_sort(v1), Vec::<i32>::new());
    }
}
