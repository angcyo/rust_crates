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
pub fn print_derive_token_stream(input: TokenStream) -> TokenStream {
    println!("print_derive_token_stream↓\n{:#?}", input); //这段代码只会在`Build`的时候输出信息，不会在`Run`的时候输出信息.
    TokenStream::new()
}

/// 带属性的过程宏
///
/// ->
/// #[derive(Debug, DerivePrintToken, DerivePrintTokenAttr)]
/// pub struct MacroStruct {
///     #[DeriveAttr]
///     name: String,
///     #[DeriveAttr(des = "描述内容", value = 100, test)]
///     pub name2: Option<String>,
/// }
///
/// <-
/// TokenStream [
///     Ident {
///         ident: "pub",
///         span: #0 bytes(485..488),
///     },
///     Ident {
///         ident: "struct",
///         span: #0 bytes(489..495),
///     },
///     Ident {
///         ident: "MacroStruct",
///         span: #0 bytes(496..507),
///     },
///     Group {
///         delimiter: Brace,
///         stream: TokenStream [
///             Punct {
///                 ch: '#',
///                 spacing: Alone,
///                 span: #0 bytes(514..515),
///             },
///             Group {
///                 delimiter: Bracket,
///                 stream: TokenStream [
///                     Ident {
///                         ident: "DeriveAttr",
///                         span: #0 bytes(516..526),
///                     },
///                 ],
///                 span: #0 bytes(515..527),
///             },
///             Ident {
///                 ident: "name",
///                 span: #0 bytes(532..536),
///             },
///             Punct {
///                 ch: ':',
///                 spacing: Alone,
///                 span: #0 bytes(536..537),
///             },
///             Ident {
///                 ident: "String",
///                 span: #0 bytes(538..544),
///             },
///             Punct {
///                 ch: ',',
///                 spacing: Alone,
///                 span: #0 bytes(544..545),
///             },
///             Punct {
///                 ch: '#',
///                 spacing: Alone,
///                 span: #0 bytes(550..551),
///             },
///             Group {
///                 delimiter: Bracket,
///                 stream: TokenStream [
///                     Ident {
///                         ident: "DeriveAttr",
///                         span: #0 bytes(552..562),
///                     },
///                     Group {
///                         delimiter: Parenthesis,
///                         stream: TokenStream [
///                             Ident {
///                                 ident: "des",
///                                 span: #0 bytes(563..566),
///                             },
///                             Punct {
///                                 ch: '=',
///                                 spacing: Alone,
///                                 span: #0 bytes(567..568),
///                             },
///                             Literal {
///                                 kind: Str,
///                                 symbol: "描述内容",
///                                 suffix: None,
///                                 span: #0 bytes(569..583),
///                             },
///                             Punct {
///                                 ch: ',',
///                                 spacing: Alone,
///                                 span: #0 bytes(583..584),
///                             },
///                             Ident {
///                                 ident: "value",
///                                 span: #0 bytes(585..590),
///                             },
///                             Punct {
///                                 ch: '=',
///                                 spacing: Alone,
///                                 span: #0 bytes(591..592),
///                             },
///                             Literal {
///                                 kind: Integer,
///                                 symbol: "100",
///                                 suffix: None,
///                                 span: #0 bytes(593..596),
///                             },
///                             Punct {
///                                 ch: ',',
///                                 spacing: Alone,
///                                 span: #0 bytes(596..597),
///                             },
///                             Ident {
///                                 ident: "test",
///                                 span: #0 bytes(598..602),
///                             },
///                         ],
///                         span: #0 bytes(562..603),
///                     },
///                 ],
///                 span: #0 bytes(551..604),
///             },
///             Ident {
///                 ident: "pub",
///                 span: #0 bytes(609..612),
///             },
///             Ident {
///                 ident: "name2",
///                 span: #0 bytes(613..618),
///             },
///             Punct {
///                 ch: ':',
///                 spacing: Alone,
///                 span: #0 bytes(618..619),
///             },
///             Ident {
///                 ident: "Option",
///                 span: #0 bytes(620..626),
///             },
///             Punct {
///                 ch: '<',
///                 spacing: Alone,
///                 span: #0 bytes(626..627),
///             },
///             Ident {
///                 ident: "String",
///                 span: #0 bytes(627..633),
///             },
///             Punct {
///                 ch: '>',
///                 spacing: Joint,
///                 span: #0 bytes(633..634),
///             },
///             Punct {
///                 ch: ',',
///                 spacing: Alone,
///                 span: #0 bytes(634..635),
///             },
///         ],
///         span: #0 bytes(508..637),
///     },
/// ]
///
#[proc_macro_derive(DerivePrintTokenAttr, attributes(DeriveAttr))]
pub fn print_derive_attr_token_stream(input: TokenStream) -> TokenStream {
    println!("print_derive_attr_token_stream↓\n{:#?}", input); //这段代码只会在`Build`的时候输出信息，不会在`Run`的时候输出信息.
    TokenStream::new()
}

/// 属性宏
///
/// ->
/// #[print_attribute_token_stream(des = "描述内容", value = 100, test)]
/// fn test_fn() {
///     println!("test_fn")
/// }
///
/// <-
/// args->TokenStream [
///     Ident {
///         ident: "des",
///         span: #0 bytes(543..546),
///     },
///     Punct {
///         ch: '=',
///         spacing: Alone,
///         span: #0 bytes(547..548),
///     },
///     Literal {
///         kind: Str,
///         symbol: "描述内容",
///         suffix: None,
///         span: #0 bytes(549..563),
///     },
///     Punct {
///         ch: ',',
///         spacing: Alone,
///         span: #0 bytes(563..564),
///     },
///     Ident {
///         ident: "value",
///         span: #0 bytes(565..570),
///     },
///     Punct {
///         ch: '=',
///         spacing: Alone,
///         span: #0 bytes(571..572),
///     },
///     Literal {
///         kind: Integer,
///         symbol: "100",
///         suffix: None,
///         span: #0 bytes(573..576),
///     },
///     Punct {
///         ch: ',',
///         spacing: Alone,
///         span: #0 bytes(576..577),
///     },
///     Ident {
///         ident: "test",
///         span: #0 bytes(578..582),
///     },
/// ]
/// input->TokenStream [
///     Ident {
///         ident: "fn",
///         span: #0 bytes(585..587),
///     },
///     Ident {
///         ident: "test_fn",
///         span: #0 bytes(588..595),
///     },
///     Group {
///         delimiter: Parenthesis,
///         stream: TokenStream [],
///         span: #0 bytes(595..597),
///     },
///     Group {
///         delimiter: Brace,
///         stream: TokenStream [
///             Ident {
///                 ident: "println",
///                 span: #0 bytes(604..611),
///             },
///             Punct {
///                 ch: '!',
///                 spacing: Alone,
///                 span: #0 bytes(611..612),
///             },
///             Group {
///                 delimiter: Parenthesis,
///                 stream: TokenStream [
///                     Literal {
///                         kind: Str,
///                         symbol: "test_fn",
///                         suffix: None,
///                         span: #0 bytes(613..622),
///                     },
///                 ],
///                 span: #0 bytes(612..623),
///             },
///         ],
///         span: #0 bytes(598..625),
///     },
/// ]
///
#[proc_macro_attribute]
pub fn print_attribute_token_stream(args: TokenStream, input: TokenStream) -> TokenStream {
    println!("print_attribute_token_stream↓");
    println!("args->{:#?}", args);
    println!("input->{:#?}", input);
    //"".parse().unwrap()
    input
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //--
    }
}
