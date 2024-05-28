// vga_buffer file

#[allow(dead_code)] //instruct the Rust compiler
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
#[repr(u8)] //define enum type
pub enum Color {   //set color bit
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(forground: Color, background: Color) -> ColorCode{
        ColorCode((background as u8) << 4 | (forground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct  ScreenChar{
    ascii_charater: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    char: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte =>{
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.char[row][col] = ScreenChar{
                    ascii_charater: byte,
                    color_code,
                }
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        
    }
}

