
#[macro_export]
macro_rules! dformat {
    ($fmt:tt $($arg:expr),*) => {
       format!(
           $crate::dm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! dprintln {
    ($fmt:tt $($arg:expr),*) => {
       println!(
           $crate::dm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! dprint {
    ($fmt:tt $($arg:expr),*) => {
       print!(
           $crate::dm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! dwrite {
    ($out:expr, $fmt:tt $($arg:expr),*) => {
       write!(
           $out,
           $crate::dm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! dwriteln {
    ($out:expr, $fmt:tt $($arg:expr),*) => {
       writeln!(
           $out,
           $crate::dm_arguments!($fmt),
           $($arg),*
       );
    };
}



#[macro_export]
macro_rules! dm_arguments {
    ([$($entry:tt)+]) => {
        concat!(
            $(deco::dm_entry!($entry)),*
        )
    };
}

#[macro_export]
macro_rules! dm_entry {
    (!) => {
        deco::dm_char!(reset)
    };

    ($name:ident) => {
        deco::dm_char!($name)
    };

    ($name:literal) => {
        $name
    }
}

#[macro_export]
macro_rules! dm_char {
    ($color:ident) => {
        concat!("\x1b[", deco::dm_charnum!($color), "m")
    }
}

#[macro_export]
macro_rules! dm_charnum {
    // Reset
    (reset) => { 0 };
    // Foreground
    (black) => { 30 };
    (red) => { 31 };
    (green) => { 32 };
    (yellow) => { 33 };
    (blue) => { 34 };
    (magenta) => { 35 };
    (cyan) => { 36 };
    (white) => { 37 };
    // Background
    (Black) => { 40 };
    (Red) => { 41 };
    (Green) => { 42 };
    (Yellow) => { 43 };
    (Blue) => { 44 };
    (Magenta) => { 45 };
    (Cyan) => { 46 };
    (White) => { 47 };
    (on_black) => { 40 };
    (on_red) => { 41 };
    (on_green) => { 42 };
    (on_yellow) => { 43 };
    (on_blue) => { 44 };
    (on_magenta) => { 45 };
    (on_cyan) => { 46 };
    (on_white) => { 47 };
    // Decoration
    (bold) => { 1 };
    (italic) => { 3 };
    (underscore) => { 4 };
    (blink) => { 5 };
    (reverse) => { 7 };
    (conceal) => { 8 };
}
