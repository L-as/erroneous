extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate synstructure;
#[macro_use]
extern crate matches2;

trait Attribute {
	const NAME: &'static str;
}

struct Source;
impl Attribute for Source {
	const NAME: &'static str = "source";
}

fn error_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
	let source_body = s.each_variant(|v| {
		v.bindings()
			.iter()
			.find(is_attr::<Source>)
			.map(|s| quote!(return Some(#s)))
			.unwrap_or(quote!(return None))
	});
	s.gen_impl(quote! {
		extern crate std;
		extern crate core;

		gen impl std::error::Error for @Self {
			#[allow(unreachable_code)]
			fn cause(&self) -> core::option::Option<&dyn std::error::Error> {
				match self {#source_body}
				None
			}
		}
	})
}

fn is_attr<Attr: Attribute>(bindings: &&synstructure::BindingInfo) -> bool {
	let mut error_meta_found = false;
	let mut found = false;
	for attr in &bindings.ast().attrs {
		if let Some(meta) = attr.interpret_meta() {
			if meta.name() == "Error" {
				assert!(
					!error_meta_found,
					"Can not have multiple #[Error]s on a field!"
				);
				let list = unwrap_match!(meta,
					syn::Meta::List(list) => list.nested, "Need parentheses!");
				let mut list = list.iter();
				let attr = unwrap_match!(list.next(),
					Some(syn::NestedMeta::Meta(syn::Meta::Word(attr))) => attr, "Content needs to be a single word!");
				assert!(list.next().is_none(), "Can only have a single word here!");
				error_meta_found = true;
				found = attr == Attr::NAME;
			}
		}
	}
	found
}

decl_derive!([Error, attributes(Error)] => error_derive);
