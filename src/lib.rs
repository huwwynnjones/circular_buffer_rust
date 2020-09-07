pub mod circular_buffer;

#[cfg(test)]
mod tests {
    use super::circular_buffer::CircularBuffer;

    #[test]
    fn test_basic_read_write() {
        let mut buff: CircularBuffer<i32> = CircularBuffer::new(5);
        let item = 3;
        buff.write(item);
        assert_eq!(buff.read(), &item)
    }

    #[test]
    fn test_overwriting() {
        let mut buff: CircularBuffer<char> = CircularBuffer::new(6);
        let input = "Hello Ada";
        let mut output = String::new();
        let correct_output = "Adalo ";
        input.chars().for_each(|c| {
            buff.write(c);
            buff.next_write()
        });
        for _ in 0..6 {
            output.push(*buff.read());
            buff.next_read()
        }
        assert_eq!(output, correct_output);
    }

    #[test]
    fn test_reader_matches_writer() {
        let mut buff: CircularBuffer<char> = CircularBuffer::new(6);
        let input = "Hello Ada";
        let mut output = String::new();
        let correct_output = "Adalo Ada";
        input.chars().for_each(|c| {
            buff.write(c);
            buff.next_write()
        });
        while !buff.reader_matches_writer() {
            output.push(*buff.read());
            buff.next_read()
        }
        assert_eq!(output, correct_output);
    }

    #[test]
    #[should_panic]
    fn test_read_pre_condition() {
        let mut buff: CircularBuffer<char> = CircularBuffer::new(6);
        let input = "Hello Ada";
        input.chars().for_each(|c| {
            buff.write(c);
            buff.next_write()
        });
        for _ in 0 .. input.len() {
            buff.next_read()
        }
        buff.next_read()
    }
}
