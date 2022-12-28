use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn hello(_attr:TokenStream, item:TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(item).unwrap();

    // Build the trait implementation
    impl_hello(&ast)
}

fn impl_hello(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        pub fn hello_attr() {
            println!("Hello, Macro! My name is {}!", stringify!(#name));
        }
    };
    gen.into()
}

