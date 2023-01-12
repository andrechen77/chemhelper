use std::collections::VecDeque;

pub struct PeekIter<I: Iterator> {
	buffer: VecDeque<<I as Iterator>::Item>,
	source: I,
}

impl<I: Iterator> PeekIter<I> {
	pub fn new(iter: I) -> Self {
		PeekIter {
			buffer: VecDeque::new(),
			source: iter,
		}
	}

	/// Attempts to ensure the buffer has the specified number of items. If the underlying iterator
	/// doesn't have enough items, fills as many as possible and returns the new buffer length.
	pub fn fill_buffer(&mut self, num_spots: usize) -> Result<(), usize> {
		for n in self.buffer.len()..num_spots {
			match self.source.next() {
				Some(item) => self.buffer.push_back(item),
				None => return Err(n),
			}
		}
		Ok(())
	}

	/// Attemps to peek the specified number of items ahead in the stream (0 is the next item). If
	/// the underlying iterator doesn't have enough items, fills the buffer with the remaining items
	/// and returns None.
	pub fn peek(&mut self, index: usize) -> Option<&<I as Iterator>::Item> {
		match self.fill_buffer(index + 1) {
			Ok(_) => Some(&self.buffer[index]),
			Err(_) => None,
		}
	}

	pub fn put_back(&mut self, item: <I as Iterator>::Item) {
		self.buffer.push_front(item);
	}

	pub fn next_if<P>(&mut self, predicate: P) -> Option<<I as Iterator>::Item>
	where
		P: FnOnce(&<I as Iterator>::Item) -> bool,
	{
		if predicate(self.peek(0)?) {
			self.next()
		} else {
			None
		}
	}
}

impl<I: Iterator> Iterator for PeekIter<I> {
	type Item = <I as Iterator>::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.buffer.len() > 0 {
			self.buffer.pop_front()
		} else {
			self.source.next()
		}
	}
}