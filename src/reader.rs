pub struct Reader<'a> {
    data: &'a [u8],
    position: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Reader {
            data,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub(crate) fn next_char(&mut self) -> Option<u8> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;

            if byte == b'\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(byte)
        } else {
            None
        }
    }

    pub(crate) fn peek(&self) -> Option<u8> {
        self.data.get(self.position).copied()
    }

    fn skip(&mut self, n: usize) {
        for _ in 0..n {
            if self.next_char().is_none() {
                break;
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.data.len()
    }
}
