/// A set of colors that can be used to set the
/// foreground or background colors on a console.
/// Custom colors may only be available on certain
/// consoles, so it is necessary to check for
/// compatibility before use.
#[deriving(Show, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Custom(uint, uint, uint)
}

/// Style modifiers for text applied on the console.
#[deriving(Show, PartialEq, Eq, Hash)]
pub enum Modifier {
    Bold,
    Underline,
    TextColor(Color),
    BackgroundColor(Color)
}

pub trait ConsoleCanvas {
    /// Draws a character to the console at the specified
    /// x, y position.
    fn draw_char(&mut self, x: uint, y: uint, c: char);

    /// Clears the screen without any garantee as to what
    /// color the screen is cleared with.
    fn clear(&mut self);

    /// Adds a modifier to the console for all text printed
    /// afterwards.
    fn set(&mut self, modifier: Modifier);

    /// Creates a new state for applying modifiers.
    fn push_state(&mut self);

    /// Returns the modifiers back to the state that they
    /// were in when `push_state()` was called.
    fn pop_state(&mut self);

    /// Returns true if this console has support for custom
    /// colors instead of the basic black, red, green, etc.
    /// If this method returns false, calling `add_modifier`
    /// with a custom color has unspecified behavior.
    fn supports_custom_colors(&self) -> bool;

    /// Swaps the back and front buffer and displays the
    /// text that has been printed to the screen using
    /// `draw_char` or `draw`.
    fn present(&mut self);

    /// Places the terminal cursor at the x, y position.
    fn cursor(&mut self, x: uint, y: uint);

    /// Returns the width of the console.
    fn width(&self) -> uint;

    /// Returns the height of the console.
    fn height(&self) -> uint;

    /// Draws a string to the console starting at position
    /// x, y and continuing to the right.
    fn draw<S: Str>(&mut self, x: uint, y: uint, text: S) {
        let text = text.as_slice();
        let mut x = x;
        for c in text.chars() {
            self.draw_char(x, y, c);
            x += 1;
        }
    }

    /// Automatically handles `push` and `pop` for
    /// the modifier state.
    fn with_state(&mut self, f: |&mut Self| -> ()) {
        self.push_state();
        f(self);
        self.pop_state();
    }
}

/// Special keys are ones that don't apply directly to
/// an ascii character.
#[deriving(Show, PartialEq, Eq, Hash)]
pub enum SpecialKey {
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Insert, Delete, Home, End, PgUp, PgDown, ArrowUp,
    ArrowDown, ArrowLeft, ArrowRight, CtrlTilde,
    CtrlPlus(char), Backspace, Tab, Enter,
    Esc, Space
}

// An update from the console.  Currently provides events
// for keyboard keys, and console resizing.
#[deriving(Show, PartialEq, Eq, Hash)]
pub enum Update {
    /// A keyboard update that translates directly
    /// to an ASCII character.
    Character(char),
    /// A keyboard update that comes from a
    /// modifier or otherwise special keyboard key.
    Special(SpecialKey),
    /// A console update triggered when the screen
    /// changes size.
    Resize(uint, uint)
}

pub trait ConsoleInput: Iterator<Update> { }
