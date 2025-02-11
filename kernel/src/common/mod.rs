use syn::{parse::Parse, Ident};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum KernelItem {
    IndepBootEntry,
    ArchBootEntry,
    SyscallSetup,
    MemMapGen,
    MemMapAlloc,
    PreuserModLoad,
    InitEnv,
    KernelFSMount,
    StorageFSMount,
    PreuserMod,
    RamLoader,
    UserInit,
    UserModLoad,
    ProcessFSMount
}

impl Parse for KernelItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        for variant in Self::iter() {
            let name = format!("{:?}", variant);

            if ident.to_string().as_str() == name {
                return Ok(variant);
            }
        }

        Err(syn::Error::new(ident.span(), "Expected one of aphrodite_common::KernelItem's variants"))
    }
}