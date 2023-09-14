//! 'Obj2Str' derive macro.

extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse_macro_input, Data, DeriveInput, Type};

/// A derive macro to implement 'Obj2Str' trait for a struct.
#[proc_macro_derive(Obj2Str)]
pub fn obj2str_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let struct_name = &ast.ident;
    let mut fields = Vec::new();

    let Data::Struct(data_struct) = ast.data else {
        panic!("Not a struct")
    };
    for field in data_struct.fields {
        fields.push((field.ident, field.ty));
    }

    let mut impl_source = String::new();

    impl_source.push_str(
        format!(
            "
    impl Obj2Str for {} {{
        fn obj2str(&self, tab_num: i8, verbosity_depth: i8) -> String {{
            if verbosity_depth <= 0 {{
                return String::from(\"{} {{...}}\");
            }}

            let mut string = String::with_capacity(1024);

            string.push_str(\"{} {{\");
    ",
            struct_name, struct_name, struct_name
        )
        .as_str(),
    );

    let mut max_name_width = 0;
    for field in fields.iter() {
        max_name_width = usize::max(max_name_width, field.0.clone().unwrap().to_string().len());
    }

    for (index, field) in fields.iter().enumerate() {
        let field_name = field.0.clone().unwrap();

        let name_width = field_name.to_string().len();
        let spaces = (name_width..max_name_width)
            .map(|_| ' ')
            .collect::<String>();

        let mut impl_field = || {
            impl_source.push_str(format!(
                "
                Self::indent(&mut string, tab_num);
                let spaces = if tab_num > 0 {{\"{}\"}} else {{\"\"}};
                string.push_str(format!(\"{}: {{}}{{}}\", spaces, self.{}.obj2str(Self::inner_tab_num(tab_num), verbosity_depth - 1)).as_str());
                if {} < {} - 1 || tab_num > 0 {{
                    string.push(',');
                }}
                ", spaces, field_name, field_name, index, fields.len()).as_str());
        };

        match field.1.clone() {
            Type::Path(_) => {
                impl_field();
            }
            Type::Tuple(_) => {
                impl_field();
            }
            _ => panic!("The type is not supported"),
        }
    }

    impl_source.push_str(
        "
            Self::indent_last(&mut string, tab_num);
            string.push('}');

            string
        }
    }
    ",
    );

    impl_source.parse().unwrap()
}
