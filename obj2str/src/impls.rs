//! Implementations of 'Obj2Str' trait for standard types.

use crate::Obj2Str;

impl Obj2Str for u8 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for u16 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for u32 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for u64 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for u128 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for usize {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for i8 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for i16 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for i32 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for i64 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for i128 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for isize {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for f32 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        format!("{:.07}", self)
    }
}

impl Obj2Str for f64 {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        format!("{:.015}", self)
    }
}

impl Obj2Str for bool {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        self.to_string()
    }
}

impl Obj2Str for char {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        format!("'{}'", self)
    }
}

impl Obj2Str for String {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        format!("\"{}\"", self)
    }
}

impl Obj2Str for () {
    fn obj2str(&self, _tab_num: i8, _verbosity_depth: i8) -> String {
        String::from("()")
    }
}

impl<T0> Obj2Str for (T0,)
where
    T0: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        let mut string = String::with_capacity(1024);

        string.push('(');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.0
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        if tab_num > 0 {
            string.push(',');
        }

        Self::indent_last(&mut string, tab_num);
        string.push(')');

        string
    }
}

impl<T0, T1> Obj2Str for (T0, T1)
where
    T0: Obj2Str,
    T1: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        let mut string = String::with_capacity(1024);

        string.push('(');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.0
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.1
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        if tab_num > 0 {
            string.push(',');
        }

        Self::indent_last(&mut string, tab_num);
        string.push(')');

        string
    }
}

impl<T0, T1, T2> Obj2Str for (T0, T1, T2)
where
    T0: Obj2Str,
    T1: Obj2Str,
    T2: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        let mut string = String::with_capacity(1024);

        string.push('(');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.0
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.1
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.2
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        if tab_num > 0 {
            string.push(',');
        }

        Self::indent_last(&mut string, tab_num);
        string.push(')');

        string
    }
}

impl<T0, T1, T2, T3> Obj2Str for (T0, T1, T2, T3)
where
    T0: Obj2Str,
    T1: Obj2Str,
    T2: Obj2Str,
    T3: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        let mut string = String::with_capacity(1024);

        string.push('(');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.0
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.1
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.2
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        string.push(',');

        Self::indent(&mut string, tab_num);
        string.push_str(
            self.3
                .obj2str(
                    Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                    verbosity_depth - 1,
                )
                .as_str(),
        );
        if tab_num > 0 {
            string.push(',');
        }

        Self::indent_last(&mut string, tab_num);
        string.push(')');

        string
    }
}

impl<T> Obj2Str for Option<T>
where
    T: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        let Some(obj) = self else {
            return "None".into();
        };

        if verbosity_depth <= 0 {
            return "Some".into();
        }

        format!(
            "Some({})",
            obj.obj2str(Self::inner_tab_num(tab_num), verbosity_depth - 1)
        )
    }
}

impl<T, U> Obj2Str for Result<T, U>
where
    T: Obj2Str,
    U: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        match self {
            Ok(obj) => {
                if verbosity_depth <= 0 {
                    return "Ok".into();
                }

                format!(
                    "Ok({})",
                    obj.obj2str(Self::inner_tab_num(tab_num), verbosity_depth - 1)
                )
            }
            Err(obj) => {
                if verbosity_depth <= 0 {
                    return "Err".into();
                }

                format!(
                    "Err({})",
                    obj.obj2str(Self::inner_tab_num(tab_num), verbosity_depth - 1)
                )
            }
        }
    }
}

impl<T> Obj2Str for Vec<T>
where
    T: Obj2Str,
{
    fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {
        if verbosity_depth <= 0 {
            return String::from(format!("[...] ({})", self.len()).as_str());
        }

        let mut string = String::with_capacity(1024);

        string.push('[');

        let max_num_width = self.len().saturating_sub(1).to_string().len();

        for index in 0..self.len() {
            Self::indent(&mut string, tab_num);
            if tab_num > 0 {
                let num_width = index.to_string().len();
                let spaces = (num_width..max_num_width).map(|_| ' ').collect::<String>();
                string.push_str(format!("({}{}) ", spaces, index).as_str());
            }
            string.push_str(
                self[index]
                    .obj2str(
                        Self::inner_tab_num(Self::inner_tab_num(tab_num)),
                        verbosity_depth - 1,
                    )
                    .as_str(),
            );
            if tab_num > 0 || index < self.len() - 1 {
                string.push(',');
            }
        }

        if !self.is_empty() {
            Self::indent_last(&mut string, tab_num);
        }
        string.push(']');

        string
    }
}
