pub struct CharacterLookup<'a> {
    buffer: &'a str,
    char_index: Box<[CharIndex]>,
    line_index: Box<[LineIndex]>,
}
impl<'a> CharacterLookup<'a> {
    pub fn new(arg: &'a str) -> CharacterLookup<'a> {
        let mut char_index = Vec::new();
        let mut line_index = Vec::new();
        let mut line_byte_start = 0usize;
        let mut line_char_start = 0usize;
        let mut buffer_index = 0usize;
        for (char_count, character) in arg.chars().enumerate() {
            char_index.push(CharIndex{
                character: character.clone(),
                byte_index: buffer_index.clone(),
                char_index: char_count.clone(),
            });
            if character == '\n' {
                // push our curent line
                line_index.push(LineIndex{
                    byte_start: line_byte_start.clone(),
                    char_start: line_char_start.clone(),
                    byte_end: buffer_index.clone(),
                    char_end: char_count.clone(),
                });
                // reset our start marker
                line_byte_start = buffer_index;
                line_char_start = char_count;
            }
            buffer_index += character.len_utf8();
        }
        CharacterLookup {
            buffer: arg,
            char_index: char_index.into_boxed_slice(),
            line_index: line_index.into_boxed_slice(),
        }
    }

    pub fn get_char(&self, index: usize) -> char {
        self.char_index[index].character.clone()
    }

    pub fn get_line_number(&self, index: usize) -> usize {
        self.line_index
            .iter()
            .enumerate()
            .filter(|(_,i)| i.char_start <= index && i.char_end >= index)
            .map(|(i,_)| i)
            .next()
            .unwrap_or(0)
    }


    pub fn get_line(&self, index: usize) -> &'a str {
        let line = self.get_line_number(index);
        let start = self.line_index[line].byte_start.clone();
        let end = self.line_index[line].byte_end.clone();
        unsafe{
            ::std::str::from_utf8_unchecked(&self.buffer.as_bytes()[start..end])
        }
    }

    /// get_span is used to return the text between character numbers start/end
    pub fn get_span(&self, start: usize, end: usize) -> &'a str {
        let start = self.char_index[start].byte_index.clone();
        let end = self.char_index[end].byte_index.clone();
        unsafe{
            ::std::str::from_utf8_unchecked(&self.buffer.as_bytes()[start..end])
        }
    }

    /// get_span_lines is used to return the "lines" between "start" and "end"
    /// this means it'll skip forward to beginning of the "start" line, and
    /// the terminator of the "end" line.
    pub fn get_span_lines(self, start: usize, end: usize) -> &'a str {
        let line_start = self.get_line_number(start);
        let start = self.line_index[line_start].byte_start.clone();
        let line_end = self.get_line_number(end);
        let end = self.line_index[line_end].byte_end.clone();
        unsafe{
            ::std::str::from_utf8_unchecked(&self.buffer.as_bytes()[start..end])
        }
    }

}

struct CharIndex {
    character: char,
    byte_index: usize,
    char_index: usize,
}

struct LineIndex {
    byte_start: usize,
    char_start: usize,
    byte_end: usize,
    char_end: usize,
}
