//! Procedural macros for QAnchor language framework

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemMod};

mod program_macro;
mod accounts_macro;

/// The `#[program]` attribute macro
/// 
/// Transforms a module into a QAnchor program with proper entry points
/// and instruction dispatch logic.
#[proc_macro_attribute]
pub fn program(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_mod = parse_macro_input!(input as ItemMod);
    
    match program_macro::expand_program(input_mod) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// The `#[derive(Accounts)]` macro
/// 
/// Generates account validation logic for instruction account structures.
#[proc_macro_derive(Accounts, attributes(account))]
pub fn derive_accounts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    match accounts_macro::expand_accounts(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
