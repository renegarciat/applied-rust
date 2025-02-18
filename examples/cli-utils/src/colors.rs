//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with ANSI escape code for green.
/// # Examples:
/// ```
/// 
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}
/// Returns a string with ANSI escape code for blue.
/// # Examples:
/// ```
/// 
/// use cli_utils::colors::*;
/// println!("{}", blue("Blue"));
/// ```

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}
/// Returns a string with ANSI escape code for bold.
/// # Examples:
/// ```
/// 
/// use cli_utils::colors::*;
/// println!("{}", bold("Bold"));
/// ```

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
/// Returns a string with ANSI escape code for reset.
/// # Examples:
/// ```
/// 
/// use cli_utils::colors::*;
/// println!("{}", reset("Reset"));
/// ```

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// 
/// `ColorString` holds the properties of a colorized string, including the string itself.
/// 
/// # Examples:
/// You can paint the string with [`ColorString::paint()`] method.
/// 
/// ```
/// let painted_string = ColorString.paint();
/// ```
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

}
