
use proc_macro::TokenStream;
use quote::quote;

use syn::{self, DeriveInput, Result};

struct MyParams(syn::Ident);

impl syn::parse::Parse for MyParams {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        //let lifetime = content.parse()?;

        let typ = content.parse()?;

        // content.parse::<Token![,]>()?;
        // let type1 = content.parse()?;
        // content.parse::<Token![,]>()?;
        // let type2 = content.parse()?;
        Ok(MyParams(typ))
    }
}


#[proc_macro_derive(TestMacro, attributes(ToAscObj))] //pub trait ToAscObj<T: ?Sized>{
pub fn test_macro_derive(tokens: TokenStream) -> TokenStream {
    //impl ToAscObj<T> for MyType

    //println!("\n\n\n\nthis is from macro:{}\n\n\n\n", tokens);


    let ast:DeriveInput = syn::parse(tokens).unwrap();
    //println!("\n\n\n\nthis is from macro:{:#?}\n\n\n\n", ast);


    let name = &ast.ident;

    let attribute = ast.attrs.iter().filter(
        |a| a.path.segments.len() == 1 && a.path.segments[0].ident == "ToAscObj"
    ).nth(0).expect("my_trait attribute required for deriving MyTrait!");

    let parameters: MyParams = syn::parse2(attribute.tokens.clone()).expect("Invalid my_trait attribute!");
    let typ = parameters.0;


    println!("\n\n\n\nthis is from macro:{:#?}\n\n\n\n", typ);

    //impl ToAscObj<T> for MyType
    let gen = quote! {
        impl runtime::ToAscObj<#typ> for #name {
            fn to(&self) {
                let nm = std::any::type_name::<#typ>();
                println!("Hello, Macro! My name is {}! Type:{}", stringify!(#name), nm);
            }
        }
    };
    println!("\n\n\n\nthis is from macro:{:#?}\n\n\n\n", gen);
  



    //impl_to_macro(&ast)    
    //TokenStream::new()
    gen.into()
}




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
      use runtime::ToAscObj;
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
    use runtime::FromAscObj;
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
