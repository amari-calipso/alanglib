use std::cmp::max;

use crate::ast::{WithPosition, SourcePos};

#[cfg(feature = "colored")]
mod color {
    use colored::{ColoredString, Colorize};

    pub fn error(msg: &str) -> ColoredString {
        msg.red()
    }

    pub fn warning(msg: &str) -> ColoredString {
        msg.bright_yellow()
    }

    pub fn note(msg: &str) -> ColoredString {
        msg.bright_blue()
    }
}

#[cfg(not(feature = "report_buf"))]
pub fn report(
    pos: &SourcePos, msg: &str, type_: &str,
    #[cfg(feature = "colored")] color: fn(&str) -> colored::ColoredString
) {
    let lines = pos.source.lines().collect::<Vec<&str>>();
    let iter_range = {
        if lines.len() < 5 {
            0..lines.len()
        } else {
            if pos.line <= 2 {
                0..5
            } else if pos.line >= lines.len() - 3 {
                (lines.len() - 5)..lines.len()
            } else {
                (pos.line - 2)..(pos.line + 3)
            }
        }
    };

    let linelen = max((iter_range.end as f64).log10().ceil() as usize, 1);

    #[cfg(feature = "colored")]
    let type_with_fmt = color(type_);
    #[cfg(not(feature = "colored"))]
    let type_with_fmt = type_;

    println!("{} ({}: line {}, pos {}): {}", type_with_fmt, pos.filename, pos.line + 1, pos.start, msg);

    for l in iter_range {
        println!("{:linelen$} | {}", l + 1, lines[l].trim_end());

        if l == pos.line {
            let arrow = "^".repeat(pos.end.saturating_sub(pos.start));

            #[cfg(feature = "colored")]
            let arrow_with_fmt = color(arrow.as_str());
            #[cfg(not(feature = "colored"))]
            let arrow_with_fmt = arrow;
            
            println!("{} | {}{}", " ".repeat(linelen), " ".repeat(pos.start), arrow_with_fmt);
        }
    }
}

#[cfg(feature = "report_buf")]
pub fn report(pos: &SourcePos, msg: &str, type_: &str) -> String {
    let lines = pos.source.lines().collect::<Vec<&str>>();

    let mut output = format!("{} ({}: line {}, pos {}): {}\n", type_, pos.filename, pos.line + 1, pos.start, msg);

    let iter_range = {
        if lines.len() < 5 {
            0..lines.len()
        } else {
            if pos.line <= 2 {
                0..5
            } else if pos.line >= lines.len() - 3 {
                (lines.len() - 5)..lines.len()
            } else {
                (pos.line - 2)..(pos.line + 3)
            }
        }
    };

    let linelen = max((iter_range.end as f64).log10().ceil() as usize, 1);

    for l in iter_range {
        output.push_str(format!("{:linelen$} | {}\n", l + 1, lines[l].trim_end()).as_ref());

        if l == pos.line {
            output.push_str(format!("{} | {}{}\n", " ".repeat(linelen), " ".repeat(pos.start), "^".repeat(pos.end.saturating_sub(pos.start))).as_ref());
        }
    }

    output
}

#[cfg(not(feature = "report_buf"))]
type Output = ();
#[cfg(feature = "report_buf")]
type Output = String;

pub fn error_astpos(pos: &SourcePos, msg: &str) -> Output {
    report(
        pos, msg, "error",
        #[cfg(feature = "colored")] color::error
    )
}

pub fn warning_astpos(pos: &SourcePos, msg: &str) -> Output {
    report(
        pos, msg, "warning",
        #[cfg(feature = "colored")] color::warning
    )
}

pub fn note_astpos(pos: &SourcePos, msg: &str) -> Output {
    report(
        pos, msg, "note",
        #[cfg(feature = "colored")] color::note
    )
}

pub fn error(item: impl WithPosition, msg: &str) -> Output {
    error_astpos(&item.get_pos(), msg)
}

pub fn warning(item: impl WithPosition, msg: &str) -> Output {
    warning_astpos(&item.get_pos(), msg)
}

pub fn note(item: impl WithPosition, msg: &str) -> Output {
    note_astpos(&item.get_pos(), msg)
}

#[macro_export]
macro_rules! error {
    ($slf: expr, $item: expr, $msg: expr) => {
        {
            $crate::report::error($item, $msg);
            $slf.errors += 1;
        }
    };
}