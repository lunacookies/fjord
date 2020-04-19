use {proc_macro::TokenStream, quote::quote};

#[proc_macro_derive(ForeignFjordFunc)]
pub fn derive(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);

    let ident = input.ident;

    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => fields,
        // Only structs are supported.
        _ => unimplemented!(),
    };

    let fields = match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => named,
        // Only named fields are supported.
        _ => unimplemented!(),
    };

    // The fields of the input struct converted from OutputExprs into their respective types, all
    // saved as bindings with the names being identical to the fields’.
    let converted_field_bindings = fields.iter().map(|f| {
        let ident = f.ident.clone();
        let ty = f.ty.clone();

        // Fields are assigned values from the list of parameters in the order that the fields were
        // defined. This will always match up, because
        //
        // - the params() method returns fields in the order that they were defined ()
        // - the parameters passed to run() are in the order that params() returns them
        quote! {
            let #ident = #ty::try_from(params.next().unwrap().val).unwrap();
        }
    });

    let field_idents = fields.iter().map(|f| f.ident.clone());

    let def_params = fields.iter().map(|f| {
        let ident = f.ident.clone();

        quote! {
            ::libfjord::params::def::Param {
                // We assume that the name of each field matches IdentName’s requirements.
                // TODO: convert from snake_case to camelCase.
                name: ::libfjord::IdentName::new_panicking(stringify!(#ident)),
                // Currently default values aren’t supported.
                default_val: ::std::option::Option::None,
            }
        }
    });

    // The name of the created struct that will implement ForeignFjordFunc.
    let func_ident = quote::format_ident!("{}Func", ident);

    let output = quote! {
        // Convert an iterator of parameters into the user-defined struct.
        impl<T> ::std::convert::From<T> for #ident
        where
            T: ::std::iter::IntoIterator<Item = ::libfjord::ffi::Param>
        {
            fn from(params: T) -> Self {
                use ::std::convert::TryFrom;

                let mut params = params.into_iter();

                #(#converted_field_bindings)*

                Self { #(#field_idents),* }
            }
        }

        #[derive(Debug)]
        pub struct #func_ident {
            def_params: ::std::vec::Vec<::libfjord::params::def::Param>,
        }

        impl #func_ident {
            pub fn new() -> Self {
                Self {
                    def_params: ::std::vec![#(#def_params),*],
                }
            }
        }

        impl ::libfjord::ffi::ForeignFjordFunc for #func_ident {
            fn params(&self) -> &[::libfjord::params::def::Param] {
                &self.def_params
            }

            fn run(
                &self,
                params: ::std::vec::Vec<::libfjord::ffi::Param>
            ) -> ::libfjord::eval::OutputExpr {
                #ident::from(params).run()
            }
        }
    };

    output.into()
}
