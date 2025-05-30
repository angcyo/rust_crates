use proc_macro::TokenStream;
use quote::quote;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/29
///

///
/// 函数调用宏
///
/// ->
/// `SELECT * FROM users WHERE age > 10`
///
/// <-
/// TokenStream [
///     Ident {
///         ident: "SELECT",
///         span: #0 bytes(186..192),
///     },
///     Punct {
///         ch: '*',
///         spacing: Alone,
///         span: #0 bytes(193..194),
///     },
///     Ident {
///         ident: "FROM",
///         span: #0 bytes(195..199),
///     },
///     Ident {
///         ident: "users",
///         span: #0 bytes(200..205),
///     },
///     Ident {
///         ident: "WHERE",
///         span: #0 bytes(206..211),
///     },
///     Ident {
///         ident: "age",
///         span: #0 bytes(212..215),
///     },
///     Punct {
///         ch: '>',
///         spacing: Alone,
///         span: #0 bytes(216..217),
///     },
///     Literal {
///         kind: Integer,
///         symbol: "10",
///         suffix: None,
///         span: #0 bytes(218..220),
///     },
/// ]
#[proc_macro]
pub fn print_token_stream(input: TokenStream) -> TokenStream {
    println!("print_token_stream↓\n{:#?}", input); //这段代码只会在`Build`的时候输出信息，不会在`Run`的时候输出信息.
    //let input_str = format!("{:#?}", input);
    //let output_input = format!("println!("{input}")");
    //r#"println!("<-请在`Build`窗口查看日志输出.(print_token_stream!)");"#.parse().unwrap()

    //--

    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    // let ast: syn::DeriveInput = syn::parse(input).unwrap();
    // let name = ast.ident;
    let generated = quote! {
        //println!("{:#?}", input_str);
        //println!("<-print_token_stream");
        println!("<-请在`Build`窗口查看日志输出.(print_token_stream!)->");
    };
    generated.into()
}

/// 过程宏
///
/// ->
/// #[derive(DerivePrintToken)]
/// struct MacroStruct {
///     name: String,
/// }
///
/// <-
/// TokenStream [
///     Ident {
///         ident: "pub",
///         span: #0 bytes(276..279),
///     },
///     Ident {
///         ident: "struct",
///         span: #0 bytes(280..286),
///     },
///     Ident {
///         ident: "MacroStruct",
///         span: #0 bytes(287..298),
///     },
///     Group {
///         delimiter: Brace,
///         stream: TokenStream [
///             Ident {
///                 ident: "name",
///                 span: #0 bytes(305..309),
///             },
///             Punct {
///                 ch: ':',
///                 spacing: Alone,
///                 span: #0 bytes(309..310),
///             },
///             Ident {
///                 ident: "String",
///                 span: #0 bytes(311..317),
///             },
///             Punct {
///                 ch: ',',
///                 spacing: Alone,
///                 span: #0 bytes(317..318),
///             },
///             Ident {
///                 ident: "pub",
///                 span: #0 bytes(323..326),
///             },
///             Ident {
///                 ident: "name2",
///                 span: #0 bytes(327..332),
///             },
///             Punct {
///                 ch: ':',
///                 spacing: Alone,
///                 span: #0 bytes(332..333),
///             },
///             Ident {
///                 ident: "Option",
///                 span: #0 bytes(334..340),
///             },
///             Punct {
///                 ch: '<',
///                 spacing: Alone,
///                 span: #0 bytes(340..341),
///             },
///             Ident {
///                 ident: "String",
///                 span: #0 bytes(341..347),
///             },
///             Punct {
///                 ch: '>',
///                 spacing: Joint,
///                 span: #0 bytes(347..348),
///             },
///             Punct {
///                 ch: ',',
///                 spacing: Alone,
///                 span: #0 bytes(348..349),
///             },
///         ],
///         span: #0 bytes(299..351),
///     },
/// ]
///
#[proc_macro_derive(DerivePrintToken)]
pub fn derive_print_token_stream(input: TokenStream) -> TokenStream {
    println!("derive_print_token_stream↓\n{:#?}", input); //这段代码只会在`Build`的时候输出信息，不会在`Run`的时候输出信息.
    let generated = quote! {
        //println!("<-请在`Build`窗口查看日志输出.(derive_print_token_stream!)->");
    };
    generated.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //--
    }
}
