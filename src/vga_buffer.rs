// vga_buffer file

//import lib
use volatile::Volatile;
//Utilities for formatting and printing strings.
use core::fmt;

//A Global Interface
//interface from other modules without carrying a Writer

use lazy_static::lazy_static;
//spinlocks is a lock that causes a thread trying to acquire it to simply wait in a loop
use spin::Mutex;
lazy_static! {
    pub static ref  WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Cyan , Color::Black),
        buffer: unsafe {
            //mutable raw pointer
            &mut *(0xb8000 as *mut Buffer)
        },
    });
}


//Colors
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

//Text Buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct  ScreenChar{
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

//Printing
impl Writer {
    //write buffer to byte
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
                //using the write method
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    
    // Writes the given ASCII string to the buffer.
    pub fn writing_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte {
                // paintable ASCII or newline not part fo printable ASCII
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                //iterate over all the screen characters and move each character one row up.
                self.buffer.chars[row - 1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize){
        // method : set all char with space char
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }
}

//create a temporary function to write some characters to the screen,
/*pub fn print_somthing(){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Cyan , Color::Black),
        buffer: unsafe {
            //mutable raw pointer
            &mut *(0xb8000 as *mut Buffer)
        },
    };
    writer.write_byte(b'H');
    writer.writing_string("ello, ");
    writer.writing_string("Welcöme to KatsuragiOS");
    //returns a Result need to unwrap to get var
    write!(writer,"This is on vga text mode  lesson {} {}",42.6969, 1.0/3.0 ).unwrap();
}*/

//rust support formatting macross easily to print different types
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.writing_string(s);
        //Ok(()) is just a Ok Contains the success value
        Ok(())
    }
}

//Macro

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}