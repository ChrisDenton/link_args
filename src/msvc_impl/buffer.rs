/// Helps to construct a list of argumets for the MSVC linker.
/// Arguments are in the form:
///
/// `/DIRECTIVE:value`
///
/// Or:
///
/// `/DIRECTIVE:value1,value2`
///
/// Multiple arguments are seperated by a space:
///
/// `/DIRECTIVE:value /DIRECTIVE:value1,value2`
pub struct Buffer<const CAPACITY: usize> {
    pub buffer: [u8; CAPACITY],
    pub len: usize,
}
#[allow(unused)]
impl<const CAPACITY: usize> Buffer<CAPACITY> {
    pub const fn new() -> Self {
        Self {
            buffer: [0; CAPACITY],
            len: 0,
        }
    }

    pub const fn push_directive(self, argument: &str) -> Self {
        self
            .push( b"/")
            .push(argument.as_bytes())
    }
    
    pub const fn push_value(self, value: &str) -> Self {
        self
            .push(b":")
            .push(value.as_bytes())
    }

    /// Turns u32's into a string such as `0x44332211`.
    /// Then pushes them as values.
    pub const fn push_values_hex(mut self, values: &[u32]) -> Self {
        if values.len() == 0 { return self; }
        let mut index = 0;
        self = self.push(b":");
        while index < values.len() - 1 {
            let hex = to_hex_u32(values[index]);
            self = self.push(&hex).push(b",");
            index += 1;
        }
        self.push(&to_hex_u32(values[index]))
    }

    pub const fn push_value_hex(self, value: u32) -> Self {
        let hex = to_hex_u32(value);
        self
            .push(b":")
            .push(&hex)
    }

    pub const fn push_value_quoted(self, value: &str) -> Self {
        if !has_quote(value.as_bytes()) {
            self
                .push(b":\"")
                .push(value.as_bytes())
                .push(b"\"")
        } else {
            self
        }
    }
    
    pub const fn push_seperator(self) -> Self {
        self.push(b" ")
    }
    
    pub const fn push(mut self, src: &[u8]) -> Self {
        let offset = self.len;
        while self.len - offset < src.len() {
            self.buffer[self.len] = src[self.len - offset];
            self.len += 1;
        }
        self
    }
}

pub const fn has_quote(s: &[u8]) -> bool {
    let mut index = 0;
    while index < s.len() {
        if s[index] == b'"' { return true; }
        index += 1;
    }
    false
}

pub const fn to_hex_u32(val: u32) -> [u8; 10] {
    let mut val = val;
    let mut bytes = *b"0x00000000";
    let lookup = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
        b'9', b'a', b'b', b'c', b'd', b'e', b'f',
    ];
    let mut index = bytes.len() - 1;
    while index > 1 {
        bytes[index] = lookup[(val & 0xf) as usize];
        val >>= 4;
        index -= 1;
    }
    bytes
}
