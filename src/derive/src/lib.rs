use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ToAscObjMacro, attributes(ToAscObj))]
pub fn to_asc_obj(tokens: TokenStream) -> TokenStream {
    // let DeriveInput { ident, .. } = parse_macro_input!(tokens);
    // let output = quote! {
    //     impl crate::ToAscObj for #ident {}
    // };
    // output.into()

    let ast = syn::parse(tokens).unwrap();
    impl_to_macro(&ast)    
}

#[proc_macro_derive(FromAscObjMacro)]
pub fn from_asc_obj(tokens: TokenStream) -> TokenStream {
    // let DeriveInput { ident, .. } = parse_macro_input!(tokens);
    // let output = quote! {
    //     impl crate::FromAscObj for #ident {}
    // };
    // output.into()

    let ast = syn::parse(tokens).unwrap();
    impl_from_macro(&ast)    
}

fn impl_to_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
      use crate::ToAscObj;
      impl ToAscObj for #name {
          fn to() {
              println!("Hello, Macro! My name is {}!", stringify!(#name));
          }
      }
  };
  gen.into()
}

fn impl_from_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    use crate::FromAscObj;
    impl FromAscObj for #name {
          fn from() {
              println!("Hello, Macro! My name is {}!", stringify!(#name));
          }
      }
  };
  gen.into()
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
