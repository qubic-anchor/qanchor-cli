//! Implementation of the #[derive(Accounts)] macro

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{
    DeriveInput, Data, Fields, Type, TypePath, Attribute, Meta, MetaList,
    Result as SynResult, Error as SynError, Ident,
};

/// Expand the #[derive(Accounts)] macro
pub fn expand_accounts(input: DeriveInput) -> SynResult<TokenStream> {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    // Only support structs
    let data = match &input.data {
        Data::Struct(data) => data,
        _ => return Err(SynError::new_spanned(
            input, 
            "Accounts can only be derived for structs"
        )),
    };
    
    // Only support named fields
    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => return Err(SynError::new_spanned(
            input,
            "Accounts struct must have named fields"
        )),
    };
    
    // Parse field constraints
    let mut field_validations = Vec::new();
    let mut field_assignments = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        
        // Parse account constraints from attributes
        let constraints = parse_account_constraints(&field.attrs)?;
        
        // Generate validation logic for this field
        let validation = generate_field_validation(field_name, field_type, &constraints)?;
        field_validations.push(validation);
        
        // Generate field assignment
        let assignment = generate_field_assignment(field_name, field_type, &constraints)?;
        field_assignments.push(assignment);
    }
    
    // Generate the Accounts implementation
    let expanded = quote! {
        impl #impl_generics qanchor_lang::accounts::Accounts<'info> for #name #ty_generics #where_clause {
            fn try_accounts(
                program_id: &'info [u8; 32],
                accounts: &mut std::iter::Peekable<std::slice::Iter<'info, qanchor_lang::context::AccountInfo<'info>>>,
                remaining_accounts: &'info [qanchor_lang::context::AccountInfo<'info>],
            ) -> std::result::Result<Self, qanchor_lang::error::ErrorCode> {
                #(#field_validations)*
                
                Ok(Self {
                    #(#field_assignments)*
                })
            }
        }
    };
    
    Ok(expanded)
}

/// Parse account constraints from field attributes
fn parse_account_constraints(attrs: &[Attribute]) -> SynResult<Vec<AccountConstraint>> {
    let mut constraints = Vec::new();
    
    for attr in attrs {
        if attr.path().is_ident("account") {
            match &attr.meta {
                Meta::List(MetaList { tokens, .. }) => {
                    // Parse constraint tokens
                    let constraint_str = tokens.to_string();
                    constraints.extend(parse_constraint_string(&constraint_str)?);
                }
                _ => return Err(SynError::new_spanned(
                    attr,
                    "Invalid account attribute format"
                )),
            }
        }
    }
    
    Ok(constraints)
}

/// Parse constraint string into constraint objects
fn parse_constraint_string(constraint_str: &str) -> SynResult<Vec<AccountConstraint>> {
    let mut constraints = Vec::new();
    
    // Simple parsing - in a real implementation, this would be more sophisticated
    for part in constraint_str.split(',') {
        let part = part.trim();
        
        if part == "init" {
            constraints.push(AccountConstraint::Init);
        } else if part == "mut" {
            constraints.push(AccountConstraint::Mut);
        } else if part == "signer" {
            constraints.push(AccountConstraint::Signer);
        } else if part.starts_with("payer") {
            if let Some(eq_pos) = part.find('=') {
                let payer = part[eq_pos + 1..].trim().to_string();
                constraints.push(AccountConstraint::Payer(payer));
            }
        } else if part.starts_with("space") {
            if let Some(eq_pos) = part.find('=') {
                let space_str = part[eq_pos + 1..].trim();
                if let Ok(space) = space_str.parse::<usize>() {
                    constraints.push(AccountConstraint::Space(space));
                }
            }
        } else if part.starts_with("owner") {
            if let Some(eq_pos) = part.find('=') {
                let owner = part[eq_pos + 1..].trim().to_string();
                constraints.push(AccountConstraint::Owner(owner));
            }
        }
    }
    
    Ok(constraints)
}

/// Generate validation logic for a field
fn generate_field_validation(
    field_name: &Ident,
    _field_type: &Type,
    constraints: &[AccountConstraint],
) -> SynResult<TokenStream> {
    let account_var = format_ident!("{}_account", field_name);
    
    // Get next account
    let mut validation = quote! {
        let #account_var = accounts.next()
            .ok_or(qanchor_lang::error::ErrorCode::InsufficientAccounts)?;
    };
    
    // Generate constraint validations
    for constraint in constraints {
        let constraint_check = match constraint {
            AccountConstraint::Mut => quote! {
                if !#account_var.is_writable {
                    return Err(qanchor_lang::error::ErrorCode::ConstraintMut);
                }
            },
            AccountConstraint::Signer => quote! {
                if !#account_var.is_signer {
                    return Err(qanchor_lang::error::ErrorCode::ConstraintSigner);
                }
            },
            AccountConstraint::Init => quote! {
                // Check that account is uninitialized
                if #account_var.data.len() > 0 {
                    return Err(qanchor_lang::error::ErrorCode::AccountAlreadyExists);
                }
            },
            AccountConstraint::Owner(owner) => {
                let owner_ident = format_ident!("{}", owner);
                quote! {
                    if #account_var.owner != &#owner_ident.key() {
                        return Err(qanchor_lang::error::ErrorCode::ConstraintOwner);
                    }
                }
            },
            AccountConstraint::Space(space) => quote! {
                if #account_var.data.len() < #space {
                    return Err(qanchor_lang::error::ErrorCode::ConstraintSpace);
                }
            },
            _ => quote! {}, // Handle other constraints
        };
        
        validation = quote! {
            #validation
            #constraint_check
        };
    }
    
    Ok(validation)
}

/// Generate field assignment logic
fn generate_field_assignment(
    field_name: &Ident,
    _field_type: &Type,
    _constraints: &[AccountConstraint],
) -> SynResult<TokenStream> {
    let account_var = format_ident!("{}_account", field_name);
    
    // Determine the field type and generate appropriate assignment
    if is_account_type(_field_type) {
        Ok(quote! {
            #field_name: qanchor_lang::accounts::Account::try_from(#account_var)?,
        })
    } else if is_signer_type(_field_type) {
        Ok(quote! {
            #field_name: qanchor_lang::accounts::Signer::try_from(#account_var)?,
        })
    } else if is_program_type(_field_type) {
        Ok(quote! {
            #field_name: qanchor_lang::accounts::Program::try_from(#account_var)?,
        })
    } else {
        Ok(quote! {
            #field_name: #account_var,
        })
    }
}

/// Check if type is Account<'info, T>
fn is_account_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Account";
        }
    }
    false
}

/// Check if type is Signer<'info>
fn is_signer_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Signer";
        }
    }
    false
}

/// Check if type is Program<'info, T>
fn is_program_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Program";
        }
    }
    false
}

/// Account constraint types
#[derive(Debug, Clone)]
enum AccountConstraint {
    Init,
    Mut,
    Signer,
    #[allow(dead_code)]
    Payer(String),
    Space(usize),
    Owner(String),
}
