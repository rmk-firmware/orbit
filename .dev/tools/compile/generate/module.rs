//# quote = "1.0"
//# proc-macro2 = "1.0"
//# syn = "2.0"
const TARGET_FILE: &str = "src/orbit.rs";

use crate::util;
use proc_macro2::Span;
use quote::quote;
use syn::Ident as SynIdent;

#[allow(unused_variables)]
pub fn generate(feature_list: &mut Vec<String>) {
  let files = util::list_files("src/orbit");

  let mut modules = vec![];
  for file in files {
    let module = util::filename_no_ext(&file);
    let module_ident = SynIdent::new(&module, Span::call_site());
    modules.push(quote! {
      pub mod #module_ident;
    });
  }

  let generated = quote! {
    #(#modules)*
  };

  util::write(TARGET_FILE, util::quote_to_string(generated).as_str());
}
