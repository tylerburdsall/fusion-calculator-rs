pub fn console_output(msg: &str) {
    eprintln!("{msg}");
}

macro_rules! output {
    () => ($crate::terminal::term::console_output(""));
    ($($a:tt)*) => {{
        $crate::terminal::term::console_output(&format!($($a)*));
    }};
}

pub(crate) use output;

macro_rules! output_err {
    ($($a:tt)*) => {{
        $crate::terminal::term::console_output(&crate::terminal::term::error(&format!($($a)*)));
    }};
}

use crossterm::style::{Attribute, Color, Stylize};
pub(crate) use output_err;

pub fn error(msg: &str) -> String {
    bold_colored_text(msg, Color::Red)
}

pub fn bold_colored_text(msg: &str, color: Color) -> String {
    let styled = msg.to_string().with(color).attribute(Attribute::Bold);

    format!("{styled}")
}
