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

pub enum Modifier {
    Bold,
    Underline,
    TextColor(Color),
    BackgroundColor(Color)
}

pub trait ConsoleCanvas {
    fn draw_char(&mut self, x: uint, y: uint, c: char);
    fn clear(&mut self);
    fn add_modifier(&mut self, modifier: &Modifier);
    fn clear_modifiers(&mut self);
    fn supports_custom_colors(&self) -> bool;
    fn present(&mut self);
    fn width(&self) -> uint;
    fn height(&self) -> uint;

    fn draw<S: Str>(&mut self, x: uint, y: uint, text: S) {
        let text = text.as_slice();
        let mut x = x;
        for c in text.chars() {
            self.draw_char(x, y, c);
            x += 1;
        }
    }

    fn with_modifiers(&mut self, modifiers: &[Modifier],
                      f: |&mut Self| -> ()) {
        for m in modifiers.iter() {
            self.add_modifier(m);
        }
        f(self);
        self.clear_modifiers();
    }
}

pub enum SpecialKey {
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Insert, Delete, Home, End, PgUp, PgDown, ArrowUp,
    ArrowDown, ArrowLeft, ArrowRight, CtrlTilde,
    CtrlPlus(char), Backspace, Tab, Enter,
    Esc, Space
}

pub enum KeyPress {
    Special(SpecialKey),
    AlphaNumeric(char)
}

pub trait ConsoleInput: Iterator<KeyPress> { }
