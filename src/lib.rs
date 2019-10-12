pub struct RomData<'a> {
    data: &'a [u8],
    pub counter: usize,
    pub offset: usize,
}

impl<'a> RomData<'a> {
    pub fn new(data: &'a [u8]) -> RomData<'a> {
        RomData {
            data,
            counter: 0,
            offset: 0,
        }
    }

    /// Print the current opcode in a human readable way.
    pub fn print_line(&self) {
        // @todo create a way to ensure vertical alignment
        // fixed length opcode->english translations?
        // some opcodes have parameters that need to be shown, do these go last?
        println!(
            "0x{:03X} {}",
            self.counter + self.offset,
            "placeholder: opcode name"
        );
    }

    /// Get the next opcode from the data stream. All chip8 opcodes are 2 bytes.
    ///
    /// # Return
    /// Returns the next opcode bytes unless at the end of the data buffer.
    fn get_next_opcode(&mut self) -> Option<u16> {
        if let Some(range) = self.data.get(self.counter..self.counter + 2) {
            self.counter += 2;
            Some((u16::from(range[0]) << 8) | u16::from(range[1]))
        } else {
            None
        }
    }

    /// Translate an opcode into a human readable format, attempt vertical
    /// alignment
    ///
    /// # Return
    /// Returns a string containing the human readable opcode
    fn translate_opcode(opcode: u16) -> &'a str {
        ""
    }

    /// Get a string containing all opcodes in the data stream.
    pub fn get_all_opcodes(&mut self) -> String {
        let mut result = String::new();
        while let Some(opcode) = self.get_next_opcode() {
            result += RomData::translate_opcode(opcode);
        }
        result
    }

    /// Get a string containing the specified number of opcodes, or until the
    /// end of the data stream.
    pub fn get_n_opcodes(&mut self) -> &'a str {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let v = vec![];
        let r = RomData::new(&v);
        assert_eq!(r.counter, 0);
        assert_eq!(r.data.len(), 0);
    }

    #[test]
    fn get_next_opcode() {
        let v = vec![0xaa, 0xff];
        let mut r = RomData::new(&v);

        assert_eq!(Some(0xaaff), r.get_next_opcode());
        assert_eq!(2, r.counter);

        assert_eq!(None, r.get_next_opcode());
    }
}
