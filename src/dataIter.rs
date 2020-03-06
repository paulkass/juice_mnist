use std::fs::File;
use std::iter::FromIterator;
use byteorder::ReadBytesExt;
use std::io::Empty;

pub struct DataIter {
	pub label_file: File,
	pub image_file: File,
	pub rows: usize,
	pub cols: usize,
}

impl DataIter {
	pub fn new(label_file: File, image_file: File, rows: usize, cols: usize) -> DataIter {
		DataIter {
			label_file,
			image_file,
			rows,
			cols
		}
	}
}

impl Iterator for DataIter {
	type Item = (u8, Vec<u8>);

	fn next(&mut self) -> Option<Self::Item> {
		let label = match self.label_file.read_u8() {
			Err(_) => return None,
			Ok(l) => l,
		};

		let image: Vec<u8> = Vec::from_iter((0..(self.rows * self.cols)).map(|_| -> u8 {
			// Have to flip bits because otherwise it will be white text on a dark background
			self.image_file.read_u8().unwrap() ^ 255
		}));

		Some((label, image))
	}
}

pub struct BatchDataIter {
	pub data_iter: DataIter,
	pub batch_size: usize,
}

impl Iterator for BatchDataIter {
	type Item = Vec<(u8, Vec<u8>)>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut result_vec: Self::Item = vec![];
		for _ in 0..self.batch_size {
			match self.data_iter.next() {
				None => break,
				Some(v) => result_vec.push(v)
			}
		}
		if result_vec.len() == 0 { None } else { Some(result_vec) }
	}
}
