use proc_macro::TokenStream;
use quote::quote;

///
/// @author <a href="mailto:angcyo@126.com">angcyo</a>
/// @date 2025/05/29
///

///
/// -> `SELECT * FROM users WHERE age > 10`
///
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
    println!("{:#?}", input); //这段代码只会在`Build`的时候输出信息，不会在`Run`的时候输出信息.
    //let input_str = format!("{:#?}", input);
    //let output_input = format!("println!("{input}")");
    r#"println!("<-请在`Build`窗口查看日志输出.(print_token_stream!)");"#.parse().unwrap()

    //--

    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    // let ast: syn::DeriveInput = syn::parse(input).unwrap();
    // let name = ast.ident;
    /*let generated = quote! {
        println!("{:#?}", input_str);
        println!("<-print_token_stream");
    };
    generated.into()*/
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //--
    }
}
