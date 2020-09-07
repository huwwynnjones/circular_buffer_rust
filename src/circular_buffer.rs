pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    read_idx: usize,
    write_idx: usize,
}

enum IdxType {
    Read,
    Write,
}

impl<T> CircularBuffer<T> {
    pub fn new(buffer_size: usize) -> CircularBuffer<T> {
        let buffer = Vec::with_capacity(buffer_size);
        let read_idx = 1;
        let write_idx = 1;
        CircularBuffer {
            buffer,
            read_idx,
            write_idx,
        }
    }

    pub fn read(&self) -> &T {
        let buffer_capacity = self.buffer.capacity();
        &self.buffer[wrap_around(buffer_capacity, self.read_idx)]
    }

    pub fn next_read(&mut self) {
        assert!(!self.reader_matches_writer());
        self.increment_idx(IdxType::Read)
    }

    pub fn next_write(&mut self) {
        self.increment_idx(IdxType::Write)
    }

    pub fn write(&mut self, item: T) {
        let buffer_capacity = self.buffer.capacity();
        if self.buffer.len() < buffer_capacity {
            self.buffer.push(item);
        } else {
            self.buffer[wrap_around(buffer_capacity, self.write_idx)] = item;
        }
    }

    pub fn reader_matches_writer(&self) -> bool {
        self.read_idx == self.write_idx
    }

    fn increment_idx(&mut self, idx_type: IdxType) {
        match idx_type {
            IdxType::Read => self.read_idx += 1,
            IdxType::Write => self.write_idx += 1,
        }
    }
}

fn wrap_around(buffer_length: usize, idx: usize) -> usize {
    let remainder = idx % buffer_length;
    if remainder == 0 {
        buffer_length - 1
    } else {
        remainder - 1
    }
}