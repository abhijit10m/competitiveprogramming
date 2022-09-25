use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_planet(&ast)
}

fn impl_planet(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Planet for #name {
            fn years_during(d: &Duration) -> f64 {
                match stringify!(#name) {
                    "Mercury" => {d.seconds as f64 / 31_556_952.0 / 0.2408467},
                    "Venus" => {d.seconds as f64 / 31_556_952.0 / 0.61519726},
                    "Earth" => {d.seconds as f64 / 31_556_952.0},
                    "Mars" => {d.seconds as f64 / 31_556_952.0 / 1.8808158},
                    "Jupiter" => {d.seconds as f64 / 31_556_952.0 / 11.862615 },
                    "Saturn" => {d.seconds as f64 / 31_556_952.0 / 29.447498},
                    "Uranus" => {d.seconds as f64 / 31_556_952.0 / 84.016846},
                    "Neptune" => {d.seconds as f64 / 31_556_952.0 / 164.79132},
                    _ => {d.seconds as f64 / 31_556_952.0},
                }
            }
        }
    };
    gen.into()
}

