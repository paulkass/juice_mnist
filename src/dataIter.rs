use std::fs::File;
use std::iter::FromIterator;
use byteorder::ReadBytesExt;

pub struct DataIter<'a> {
	labelFile: &'a mut File,
	imageFile: &'a mut File,
	rows: usize,
	cols: usize,
}

impl DataIter<'_> {
	pub fn new<'a>(labelFile: &'a mut File, imageFile: &'a mut File, rows: usize, cols: usize) -> DataIter<'a> {
		DataIter {
			labelFile,
			imageFile,
			rows,
			cols
		}
	}
}

impl Iterator for DataIter<'_> {
	type Item = (u8, Vec<u8>);

	fn next(&mut self) -> Option<Self::Item> {
		let label = match self.labelFile.read_u8() {
			Err(_) => return None,
			Ok(l) => l,
		};

		let image: Vec<u8> = Vec::from_iter((0..(self.rows * self.cols)).map(|_| -> u8 {
			// Have to flip bits because otherwise it will be white text on a dark background
			self.imageFile.read_u8().unwrap() ^ 255
		}));

		Some((label, image))
	}
}

pub struct BatchDataIter<'a> {
	pub dataIter: DataIter<'a>,
	pub batch_size: usize,
}

impl Iterator for BatchDataIter<'_> {
	type Item = Vec<(u8, Vec<u8>)>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut result_vec: Self::Item = vec![];
		for _ in 0..10 {
			match self.dataIter.next() {
				None => break,
				Some(v) => result_vec.push(v)
			}
		}
		if result_vec.len() == 0 { None } else { Some(result_vec) }
	}
}
