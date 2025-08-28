//! Implementation of the #[program] attribute macro

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{
    ItemMod, ItemFn, FnArg, PatType, Type, TypePath,
    Result as SynResult, Error as SynError,
};

/// Expand the #[program] attribute macro
pub fn expand_program(
    input_mod: ItemMod,
) -> SynResult<TokenStream> {
    let mod_name = &input_mod.ident;
    let mod_vis = &input_mod.vis;
    
    // Extract the module content
    let mod_name_for_error = &input_mod.ident;
    let content = input_mod.content
        .ok_or_else(|| SynError::new_spanned(mod_name_for_error, "Program module must have a body"))?;
    
    let items = content.1;
    
    // Find all public functions (instruction handlers)
    let mut instruction_handlers = Vec::new();
    let mut other_items = Vec::new();
    
    for item in items {
        if let syn::Item::Fn(item_fn) = &item {
            if matches!(item_fn.vis, syn::Visibility::Public(_)) {
                instruction_handlers.push(item_fn.clone());
            }
        }
        other_items.push(item);
    }
    
    // Generate the program struct
    let program_struct_name = format_ident!("{}Program", 
        mod_name.to_string()
            .chars()
            .enumerate()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect::<String>()
    );
    
    // Generate instruction enum
    let instruction_enum = generate_instruction_enum(&instruction_handlers)?;
    
    // Generate instruction dispatch logic
    let dispatch_logic = generate_dispatch_logic(&instruction_handlers)?;
    
    // Generate the complete program implementation
    let expanded = quote! {
        #mod_vis mod #mod_name {
            use super::*;
            use qanchor_lang::prelude::*;
            
            #(#other_items)*
            
            // Generated instruction enum
            #instruction_enum
            
            // Program struct
            pub struct #program_struct_name;
            
            impl qanchor_lang::program::ProgramEntry for #program_struct_name {
                fn execute(
                    program_id: &[u8; 32],
                    accounts: &[qanchor_lang::context::AccountInfo],
                    instruction_data: &[u8],
                ) -> std::result::Result<(), qanchor_lang::error::ErrorCode> {
                    use qanchor_lang::error::Result;
                    
                    // Try to deserialize instruction from instruction_data
                    let instruction: ProgramInstruction = 
                        qanchor_lang::program::InstructionParams::extract(instruction_data)?;
                    
                    // Dispatch to appropriate handler
                    #dispatch_logic
                }
            }
            
            // Generate entry point
            qanchor_lang::program_entry!(#program_struct_name);
        }
    };
    
    Ok(expanded)
}

/// Generate the instruction enum from handler functions
fn generate_instruction_enum(handlers: &[ItemFn]) -> SynResult<TokenStream> {
    let mut variants = Vec::new();
    
    for handler in handlers {
        let handler_name = &handler.sig.ident;
        let variant_name = format_ident!("{}",
            handler_name.to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        );
        
        // Extract parameters (excluding Context parameter)
        let mut params = Vec::new();
        for input in &handler.sig.inputs {
            if let FnArg::Typed(PatType { ty, .. }) = input {
                // Skip Context parameter
                if !is_context_type(ty) {
                    params.push(ty);
                }
            }
        }
        
        if params.is_empty() {
            variants.push(quote! { #variant_name });
        } else {
            // Use tuple variant syntax instead of struct variant
            variants.push(quote! { #variant_name(#(#params),*) });
        }
    }
    
    Ok(quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        pub enum ProgramInstruction {
            #(#variants),*
        }
    })
}

/// Generate the instruction dispatch logic
fn generate_dispatch_logic(handlers: &[ItemFn]) -> SynResult<TokenStream> {
    let mut match_arms = Vec::new();
    
    for handler in handlers {
        let handler_name = &handler.sig.ident;
        let variant_name = format_ident!("{}",
            handler_name.to_string()
                .chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        );
        
        // Find the Context parameter and other parameters
        let mut context_type = None;
        let mut param_names = Vec::new();
        let mut param_count = 0;
        
        for input in &handler.sig.inputs {
            if let FnArg::Typed(PatType { pat, ty, .. }) = input {
                if is_context_type(ty) {
                    context_type = Some(extract_context_accounts_type(ty)?);
                } else {
                    if let syn::Pat::Ident(pat_ident) = &**pat {
                        param_names.push(&pat_ident.ident);
                        param_count += 1;
                    }
                }
            }
        }
        
        let context_accounts_type = context_type
            .ok_or_else(|| SynError::new_spanned(handler, 
                "Instruction handler must have a Context parameter"))?;
        
        let dispatch_call = if param_count == 0 {
            // No additional parameters
            quote! {
                ProgramInstruction::#variant_name => {
                    // Create account iterator and validate accounts
                    let mut account_iter = accounts.iter().peekable();
                    let validated_accounts = #context_accounts_type::try_accounts(
                        program_id,
                        &mut account_iter,
                        &[],
                    )?;
                    let ctx = qanchor_lang::context::Context::new(validated_accounts, program_id, &[]);
                    #handler_name(ctx)
                }
            }
        } else {
            // Has additional parameters - extract from tuple variant
            quote! {
                ProgramInstruction::#variant_name(#(#param_names),*) => {
                    // Create account iterator and validate accounts
                    let mut account_iter = accounts.iter().peekable();
                    let validated_accounts = #context_accounts_type::try_accounts(
                        program_id,
                        &mut account_iter,
                        &[],
                    )?;
                    let ctx = qanchor_lang::context::Context::new(validated_accounts, program_id, &[]);
                    #handler_name(ctx, #(#param_names),*)
                }
            }
        };
        
        match_arms.push(dispatch_call);
    }
    
    Ok(quote! {
        match instruction {
            #(#match_arms)*
        }
    })
}

/// Check if a type is a Context type
fn is_context_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Context";
        }
    }
    false
}

/// Extract the accounts type from a Context<T> type
fn extract_context_accounts_type(ty: &Type) -> SynResult<&Type> {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            if segment.ident == "Context" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(accounts_type)) = args.args.first() {
                        return Ok(accounts_type);
                    }
                }
            }
        }
    }
    Err(SynError::new_spanned(ty, "Invalid Context type"))
}
