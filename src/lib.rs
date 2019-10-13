use std::string::String;

// Opcode.opcode only used in tests
#[allow(dead_code)]
struct Opcode {
    opcode: u16,
    address: u16,
    constant: u16,
    n1: u16,
    n2: u16,
    n3: u16,
    n4: u16,
}

impl Opcode {
    pub fn new(opcode: u16) -> Opcode {
        Opcode {
            opcode,
            address: opcode & 0x0FFF,
            constant: opcode & 0x00FF,
            n1: (opcode & 0xF000) >> 12,
            n2: (opcode & 0x0F00) >> 8,
            n3: (opcode & 0x00F0) >> 4,
            n4: opcode & 0x000F,
        }
    }
}

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

    fn next(&mut self) {
        self.counter += 2;
    }

    /// Convenience function to get the memory address of an opcode
    fn addr(&self) -> usize {
        self.counter + self.offset
    }

    /// Get the next opcode from the data stream. All chip8 opcodes are 2 bytes.
    ///
    /// # Return
    /// Returns the next opcode bytes unless at the end of the data buffer.
    fn get_next_opcode(&mut self) -> Option<Opcode> {
        if let Some(range) = self.data.get(self.counter..self.counter + 2) {
            Some(Opcode::new(
                (u16::from(range[0]) << 8) | u16::from(range[1]),
            ))
        } else {
            None
        }
    }

    /// Translate an opcode into a human readable format, attempt vertical
    /// alignment
    ///
    /// # Return
    /// Returns a string containing the human readable opcode
    fn translate_opcode(&self, opcode: Opcode) -> String {
        match opcode.n1 {
            0x0 => match opcode.n4 {
                0x0 => format!("0x0{:03X} CLS", self.addr()),
                0xE => format!("0x0{:03X} RET", self.addr()),
                _ => format!(
                    "0x0{:03X} ???       0x{:04X}",
                    self.addr(),
                    opcode.opcode
                ),
            },
            0x1 => format!(
                "0x0{:03X} JMP       0x0{:03X}",
                self.addr(),
                opcode.address
            ),
            0x2 => format!(
                "0x0{:03X} CALL      0x0{:03X}",
                self.addr(),
                opcode.address
            ),
            0x3 => format!(
                "0x0{:03X} SKIP_EQ   V{:X} 0x0{:03X}",
                self.addr(),
                opcode.n2,
                opcode.constant
            ),
            0x4 => format!(
                "0x0{:03X} SKIP_NE   V{:X} 0x0{:03X}",
                self.addr(),
                opcode.n2,
                opcode.constant
            ),
            0x5 => format!(
                "0x0{:03X} SKIP_EQ   V{:X} V{:X}",
                self.addr(),
                opcode.n2,
                opcode.n3
            ),
            0x6 => format!(
                "0x0{:03X} LOAD      V{:X} 0x0{:03X}",
                self.addr(),
                opcode.n2,
                opcode.constant
            ),
            0x7 => format!(
                "0x0{:03X} ADD       V{:X} 0x0{:03X}",
                self.addr(),
                opcode.n2,
                opcode.constant
            ),
            0x8 => match opcode.n4 {
                0x0 => format!(
                    "0x0{:03X} LOAD      V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x1 => format!(
                    "0x0{:03X} OR        V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x2 => format!(
                    "0x0{:03X} AND       V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x3 => format!(
                    "0x0{:03X} XOR       V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x4 => format!(
                    "0x0{:03X} ADD       V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x5 => format!(
                    "0x0{:03X} SUB       V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x6 => format!(
                    "0x0{:03X} SHIFT_R   V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                0x7 => format!(
                    "0x0{:03X} SUB       V{:X} V{:X}",
                    self.addr(),
                    opcode.n3,
                    opcode.n2
                ),
                0xE => format!(
                    "0x0{:03X} SHIFT_L   V{:X} V{:X}",
                    self.addr(),
                    opcode.n2,
                    opcode.n3
                ),
                _ => format!(
                    "0x0{:03X} ???       0x{:04X}",
                    self.addr(),
                    opcode.opcode
                ),
            },
            0x9 => format!(
                "0x0{:03X} SKIP_NE   V{:X} V{:X}",
                self.addr(),
                opcode.n2,
                opcode.n3
            ),
            0xA => format!(
                "0x0{:03X} LOAD      I  0x0{:03X}",
                self.addr(),
                opcode.address
            ),
            0xB => format!(
                "0x0{:03X} JUMP      0x0{:03X}",
                self.addr(),
                opcode.address
            ),
            0xC => format!(
                "0x0{:03X} RND       V{:X} 0x0{:03X}",
                self.addr(),
                opcode.n2,
                opcode.constant
            ),
            0xD => format!(
                "0x0{:03X} DRAW      V{:X} V{:X} {:X}",
                self.addr(),
                opcode.n2,
                opcode.n3,
                opcode.n4
            ),
            0xE => match opcode.n3 {
                0x9 => {
                    format!("0x0{:03X} SKIP_KP   V{:X}", self.addr(), opcode.n2)
                }
                0xA => {
                    format!("0x0{:03X} SKIP_NKP  V{:X}", self.addr(), opcode.n2)
                }
                _ => format!(
                    "0x0{:03X} ???       0x{:04X}",
                    self.addr(),
                    opcode.opcode
                ),
            },
            0xF => match opcode.n3 {
                0x0 => match opcode.n4 {
                    0x7 => format!(
                        "0x0{:03X} LOAD_DT   V{:X} DT",
                        self.addr(),
                        opcode.n2
                    ),
                    0xA => format!(
                        "0x0{:03X} LOAD_KEY  V{:X} KEY",
                        self.addr(),
                        opcode.n2
                    ),
                    _ => format!(
                        "0x0{:03X} ???       0x{:04X}",
                        self.addr(),
                        opcode.opcode
                    ),
                },
                0x1 => match opcode.n4 {
                    0x5 => format!(
                        "0x0{:03X} SET_DT    V{:X} DT",
                        self.addr(),
                        opcode.n2
                    ),
                    0x8 => format!(
                        "0x0{:03X} SET_ST    V{:X} DT",
                        self.addr(),
                        opcode.n2
                    ),
                    0xE => format!(
                        "0x0{:03X} ADD       I  V{:X}",
                        self.addr(),
                        opcode.n2
                    ),
                    _ => format!(
                        "0x0{:03X} ???       0x{:04X}",
                        self.addr(),
                        opcode.opcode
                    ),
                },
                0x2 => {
                    format!("0x0{:03X} LOAD_FONT V{:X}", self.addr(), opcode.n2)
                }
                0x3 => {
                    format!("0x0{:03X} BCD       V{:X}", self.addr(), opcode.n2)
                }
                0x5 => format!(
                    "0x0{:03X} STORE_REG V0 - V{:X}",
                    self.addr(),
                    opcode.n2
                ),
                0x6 => format!(
                    "0x0{:03X} LOAD_REG  V0 - V{:X}",
                    self.addr(),
                    opcode.n2
                ),
                _ => format!(
                    "0x0{:03X} ???       0x{:04X}",
                    self.addr(),
                    opcode.opcode
                ),
            },
            _ => format!(
                "0x0{:03X} ???       0x{:04X}",
                self.addr(),
                opcode.opcode
            ),
        }
    }
    /// Get a string containing all opcodes in the data stream.
    pub fn get_all_opcodes(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        while let Some(opcode) = self.get_next_opcode() {
            result.push(self.translate_opcode(opcode));
            self.next();
        }
        result
    }

    /// Get a string containing the specified number of opcodes, or until the
    /// end of the data stream.
    pub fn get_n_opcodes(&mut self, start: usize, n: usize) -> Vec<String> {
        let mut result = Vec::new();
        for _ in start..start + n {
            if let Some(opcode) = self.get_next_opcode() {
                result.push(self.translate_opcode(opcode));
                self.next();
            } else {
                break;
            }
        }
        result
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
    fn get_n_opcodes_no_overflow() {
        let v = vec![0x00, 0xE0, 0x00, 0xEE];
        let mut r = RomData::new(&v);
        let s = r.get_n_opcodes(0, 1);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "0x0000 CLS");
    }

    #[test]
    fn get_n_opcodes_overflow() {
        let v = vec![0x00, 0xE0, 0x00, 0xEE];
        let mut r = RomData::new(&v);
        let s = r.get_n_opcodes(0, 3);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "0x0000 CLS");
        assert_eq!(s[1], "0x0002 RET");
    }

    #[test]
    fn get_two_opcodes() {
        let v = vec![0x00, 0xE0, 0x00, 0xEE];
        let mut r = RomData::new(&v);
        let s = r.get_all_opcodes();
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "0x0000 CLS");
        assert_eq!(s[1], "0x0002 RET");
    }

    #[test]
    fn translate_opcode() {
        let v = vec![0x00, 0xE0];
        let mut r = RomData::new(&v);
        let opcode = r.get_next_opcode();
        assert_eq!(r.translate_opcode(opcode.unwrap()), "0x0000 CLS");
    }

    #[test]
    fn get_next_opcode_valid() {
        let v = vec![0xAA, 0xFF];
        let mut r = RomData::new(&v);

        assert_eq!(0xaaff, r.get_next_opcode().unwrap().opcode);
        assert_eq!(0, r.counter);
    }

    #[test]
    fn get_next_opcode_invalid() {
        let v = vec![0xAA, 0xFF];
        let mut r = RomData::new(&v);
        r.counter = 2;
        assert!(r.get_next_opcode().is_none());
    }

    #[test]
    fn next() {
        let v = vec![];
        let mut r = RomData::new(&v);
        assert_eq!(r.counter, 0);
        r.next();
        assert_eq!(r.counter, 2);
    }
}
