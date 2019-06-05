
#[macro_export]
macro_rules! tcformat {
    ($fmt:tt $($arg:expr),*) => {
       format!(
           $crate::tcm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! tcprintln {
    ($fmt:tt $($arg:expr),*) => {
       println!(
           $crate::tcm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! tcprint {
    ($fmt:tt $($arg:expr),*) => {
       print!(
           $crate::tcm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! tcwrite {
    ($out:expr, $fmt:tt $($arg:expr),*) => {
       write!(
           $out,
           $crate::tcm_arguments!($fmt),
           $($arg),*
       );
    };
}

#[macro_export]
macro_rules! tcwriteln {
    ($out:expr, $fmt:tt $($arg:expr),*) => {
       writeln!(
           $out,
           $crate::tcm_arguments!($fmt),
           $($arg),*
       );
    };
}



#[macro_export]
macro_rules! tcm_arguments {
    ([$($entry:tt)+]) => {
        concat!(
            $(deco::tcm_entry!($entry)),*
        )
    };
}

#[macro_export]
macro_rules! tcm_entry {
    (!) => {
        deco::tcm_char!(reset)
    };

    ($name:ident) => {
        deco::tcm_char!($name)
    };

    ($name:literal) => {
        $name
    }
}

#[macro_export]
macro_rules! tcm_char {
    ($color:ident) => {
        concat!("\x1b[", deco::tcm_charnum!($color), "m")
    }
}

#[macro_export]
macro_rules! tcm_charnum {
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
    // Decoration
    (bold) => { 1 };
    (italic) => { 3 };
    (underscore) => { 4 };
    (blink) => { 5 };
    (reverse) => { 7 };
    (conceal) => { 8 };
}
