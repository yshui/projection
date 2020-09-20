#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use ::darling::{FromDeriveInput, FromField, FromMeta};
use ::proc_macro::{Diagnostic, Level, TokenStream};
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::{format_ident, quote};
use ::syn::{parse_macro_input, AttributeArgs, DeriveInput};

#[derive(Debug, FromMeta)]
#[darling(default)]
struct ProjectionArgs {
	option: bool,
	result: bool,
}

#[derive(FromField)]
#[darling(forward_attrs)]
struct ProjectionField {
	ident: Option<::syn::Ident>,
	vis: ::syn::Visibility,
	ty: ::syn::Type,
	//attrs: Vec<::syn::Attribute>,
}

#[derive(FromDeriveInput)]
#[darling(supports(struct_named))]
struct ProjectionStruct {
	ident: ::syn::Ident,
	data: ::darling::ast::Data<(), ProjectionField>,
	vis: ::syn::Visibility,
}

impl Default for ProjectionArgs {
	fn default() -> Self {
		Self {
			option: true,
			result: true,
		}
	}
}

fn generate_option_impl(p: &ProjectionStruct, out: &mut TokenStream2) {
	let orig_name = p.ident.clone();
	let struct_name_base = format_ident!("__projection_internal_Option_{}_projected", p.ident);
	let vis = &p.vis;
	let mut generate = |for_mut, for_ref| {
		use ::darling::ast::Data::*;
		let fields = match &p.data {
			Enum(_) => panic!(),
			Struct(s) => s,
		};
		let struct_name = if for_mut {
			format_ident!("{}_ref_mut", struct_name_base)
		} else if for_ref {
			format_ident!("{}_ref", struct_name_base)
		} else {
			struct_name_base.clone()
		};
		let mut_token = if for_mut {
			quote! { mut }
		} else {
			quote! {}
		};
		let ref_token = if for_ref {
			quote! { & }
		} else {
			quote! {}
		};
		let lifetime_token = if for_ref {
			quote! { 'a }
		} else {
			quote! {}
		};
		let projected_fields: TokenStream2 = fields
			.iter()
			.map(|f| {
				let ident = f.ident.as_ref().unwrap();
				let ty = &f.ty;
				let vis = &f.vis;
				quote! { #vis #ident : Option<#ref_token #lifetime_token #mut_token #ty>, }
			})
			.collect();
		let projected_fields_init: TokenStream2 = fields
			.iter()
			.map(|f| {
				let ident = f.ident.as_ref().unwrap();
				quote! { #ident: Some(#ref_token #mut_token f.#ident), }
			})
			.collect();
		out.extend(quote! {
			#[derive(Default)]
			#[allow(non_camel_case_types)]
			#vis struct #struct_name<#lifetime_token> {
				#projected_fields
			}
			impl<#lifetime_token> From<#ref_token #lifetime_token #mut_token #orig_name> for
			    #struct_name<#lifetime_token> {
				fn from(f: #ref_token #lifetime_token #mut_token #orig_name) -> Self {
					Self {
						#projected_fields_init
					}
				}
			}

			impl<#lifetime_token> ::projection::OptionProjectable for
			    #ref_token #lifetime_token #mut_token #orig_name {
				type P = #struct_name<#lifetime_token>;
				fn project(f: Option<Self>) -> Self::P {
					match f {
						Some(t) => t.into(),
						None => Default::default(),
					}
				}
			}
		});
	};
	generate(false, true);
	generate(true, true);
	generate(false, false);
}

fn generate_result_impl(_p: &ProjectionStruct, _out: &mut TokenStream2) {}

#[proc_macro_attribute]
pub fn projection(args: TokenStream, input: TokenStream) -> TokenStream {
	fn imp(attr: AttributeArgs, input: DeriveInput) -> Result<TokenStream, ::darling::Error> {
		use ::quote::ToTokens;
		let project = ProjectionStruct::from_derive_input(&input)?;
		let project_args = ProjectionArgs::from_list(&attr)?;
		let mut ret = TokenStream::new().into();
		if project_args.option {
			generate_option_impl(&project, &mut ret);
		}
		if project_args.result {
			generate_result_impl(&project, &mut ret);
		}
		input.to_tokens(&mut ret);
		Ok(ret.into())
	}
	let attr = parse_macro_input!(args as AttributeArgs);
	let input = parse_macro_input!(input as DeriveInput);
	match imp(attr, input) {
		Ok(stream) => stream,
		Err(e) => {
			Diagnostic::new(Level::Error, e.to_string()).emit();
			TokenStream::new()
		}
	}
}
