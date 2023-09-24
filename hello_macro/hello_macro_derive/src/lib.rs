use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate/
    // We clone for printing; that's the only reason.
    let ast = syn::parse(input.clone()).expect("Expected to be able to parse input.");

    // This gets printed during compile time, not during runtime!
    // struct PancakesTest ;
    println!("input:\n{}", input);
    /*
    TokenStream [
      Ident { ident: "struct", span: #0 bytes(162..168) },
      Ident { ident: "PancakesTest", span: #0 bytes(169..181) },
      Punct { ch: ';', spacing: Alone, span: #0 bytes(181..182) }
    ]
    */
    println!("input:\n{:?}", input);

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // This gets printed during compile time, not during runtime!
    // Ident { ident: "PancakesTest", span: #0 bytes(169..181) }
    println!("name:\n{:?}", name);
    // PancakesTest
    println!("name:\n{}", name);

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // gen.into()

    let gen_into = gen.into();
    // This gets printed during compile time, not during runtime!
    println!("gen_into:\n{:?}", gen_into);
    gen_into
}

/*
println!("gen_into:\n{}", gen_into);

gen_into:
impl HelloMacro for PancakesTest
{
    fn hello_macro()
    { println! ("Hello, Macro! My name is {}!", stringify! (PancakesTest)) ; }
}



println!("gen_into:\n{:?}", gen_into);

gen_into:
TokenStream [
  Ident { ident: "impl", span: #5 bytes(145..155) },
  Ident { ident: "HelloMacro", span: #5 bytes(145..155) },
  Ident { ident: "for", span: #5 bytes(145..155) },
  Ident { ident: "PancakesTest", span: #0 bytes(169..181) },
  Group { delimiter: Brace, stream: TokenStream [
    Ident { ident: "fn", span: #5 bytes(145..155) },
    Ident { ident: "hello_macro", span: #5 bytes(145..155) },
    Group { delimiter: Parenthesis, stream: TokenStream [], span: #5 bytes(145..155) },
    Group { delimiter: Brace, stream: TokenStream [
      Ident { ident: "println", span: #5 bytes(145..155) },
      Punct { ch: '!', spacing: Alone, span: #5 bytes(145..155) },
      Group { delimiter: Parenthesis, stream: TokenStream [
        Literal { kind: Str, symbol: "Hello, Macro! My name is {}!", suffix: None, span: #5 bytes(145..155) },
        Punct { ch: ',', spacing: Alone, span: #5 bytes(145..155) },
        Ident { ident: "stringify", span: #5 bytes(145..155) },
        Punct { ch: '!', spacing: Alone, span: #5 bytes(145..155) },
        Group { delimiter: Parenthesis, stream: TokenStream [
          Ident { ident: "PancakesTest", span: #0 bytes(169..181) }
        ], span: #5 bytes(145..155)
        }
      ], span: #5 bytes(145..155)
      },
      Punct { ch: ';', spacing: Alone, span: #5 bytes(145..155) }
    ], span: #5 bytes(145..155)
    }
  ], span: #5 bytes(145..155)
  }
]
*/
