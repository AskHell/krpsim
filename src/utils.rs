use std::fmt::Debug;

pub fn generalize_error<T: Debug>(err: T) -> String {
	format!("{:?}", err)
}
