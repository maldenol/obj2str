//! 'Obj2Str' trait.

mod impls;

/// A trait to convert an object to a string.
pub trait Obj2Str {
    /// Converts the object to a string.
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String;

    /// Adds proper indentation to a string.
    fn indent(string: &mut String, tab_num: i8) {
        if tab_num > 0 {
            string.push('\n');
            for _ in 0..tab_num {
                string.push('\t');
            }
        } else {
            string.push(' ');
        }
    }

    /// Adds proper indentation to a string (for the last item).
    fn indent_last(string: &mut String, tab_num: i8) {
        if tab_num > 0 {
            string.push('\n');
            for _ in 0..(tab_num - 1) {
                string.push('\t');
            }
        } else {
            string.push(' ');
        }
    }

    /// Returns tabulations number for an inner item.
    fn inner_tab_num(tab_num: i8) -> i8 {
        if tab_num > 0 {
            tab_num + 1
        } else {
            0
        }
    }
}
