extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{
	parse::{Error, Result},
	spanned::Spanned,
	Data, DeriveInput, Fields, Meta, NestedMeta,
};

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	let name = &input.ident;
	let arms = match get_match_arms(&input) {
	    Ok(arms) => arms,
	    Err(e) => return e.to_compile_error().into(),
	};

	let assertion = Ident::new(&format!("_ErrorDeriveAssertBoundsFor{}", name), Span::call_site());

	let predicates: TokenStream = where_clause
		.iter()
		.flat_map(|w| w.predicates.iter())
		.map(|p| quote!(#p,))
		.chain(Some(quote!(#name #ty_generics: Send + Sync + Sized + 'static)))
		.collect();

	let assert_bounds = quote_spanned! {input.span()=>
		struct #assertion #impl_generics (#name #ty_generics) where #predicates;
	};

	let expanded = quote! {
	    #assert_bounds
		impl #impl_generics ::std::error::Error for #name #ty_generics #where_clause {
			fn source(&self) -> core::option::Option<&(dyn std::error::Error + 'static)> {
				match self {
					#arms
				}
			}
		}
	};

	expanded.into()
}

fn get_matcher(fields: &Fields) -> TokenStream {
	match fields {
		Fields::Unit => TokenStream::new(),
		Fields::Unnamed(fields) => {
			let fields: TokenStream = (0..fields.unnamed.len())
				.map(|n| {
					let i = Ident::new(&format!("_{}", n), Span::call_site());
					quote!(#i,)
				}).collect();
			quote!((#fields))
		}
		Fields::Named(fields) => {
			let fields: TokenStream = fields
				.named
				.iter()
				.map(|f| {
					let i = f.ident.as_ref().unwrap();
					quote!(#i,)
				}).collect();
			quote!({#fields})
		}
	}
}

fn get_expr(fields: &Fields) -> Result<TokenStream> {
	const PROPER_SYNTAX: &'static str = "Proper syntax: #[error(source)] my_field";

	let mut source = None;
	for (i, field) in fields.iter().enumerate() {
		if field.attrs
			.iter()
			.filter_map(|a| a.interpret_meta())
			.filter(|m| m.name() == "error")
			.map(|m| match m {
				Meta::List(list) => Ok(list),
				m => Err(Error::new(m.span(), PROPER_SYNTAX))
			})
			.map(|m| {
				let list = m?.nested;
				if list.len() != 1 {
					Err(Error::new(list.span(), PROPER_SYNTAX))
				} else {
					Ok(list[0].clone()) // can't move out... why no IndexMove?
				}
			})
			.map(|nested| match nested? {
				NestedMeta::Meta(Meta::Word(ident)) => match ident.to_string().as_ref() {
					"source" => Ok(()),
					_ => Err(Error::new(ident.span(), "Only #[error(source)] is supported")),
				},
				nested => Err(Error::new(nested.span(), PROPER_SYNTAX)),
			})
			.try_fold(false, |s, r| {
				if s {
					Err(Error::new(field.span(), "Too many attributes!"))
				} else {
					r?;
					Ok(true)
				}
			})?
		{
			if source.is_some() {
				return Err(Error::new(fields.span(), "Too many sources, there can only be 1!"));
			}
			source = Some((i, field));
		}
	}

	Ok(match source {
		Some((i, field)) => if let Some(ident) = &field.ident {
			quote!(Some(#ident))
		} else {
			let ident = Ident::new(&format!("_{}", i), Span::call_site());
			quote!(Some(#ident))
		},
		None => quote!(None),
	})
}

fn get_match_arms(input: &DeriveInput) -> Result<TokenStream> {
	match &input.data {
		Data::Enum(e) => e.variants.iter().try_fold(TokenStream::new(), |arms, v| {
			let matcher = get_matcher(&v.fields);
			let expr = get_expr(&v.fields)?;
			let name = &input.ident;
			let v_name = &v.ident;
			Ok(quote!(#arms #name::#v_name #matcher => #expr,))
		}),
		Data::Struct(s) => {
			let matcher = get_matcher(&s.fields);
			let expr = get_expr(&s.fields)?;
			let name = &input.ident;
			Ok(quote!(#name #matcher => #expr,))
		},
		Data::Union(_) => {
			Err(Error::new(input.span(), "Can not derive Error on unions"))
		},
	}
}
