mod hook_resolve;
pub(crate) use hook_resolve::resolve_name_hooks;
mod pluralize;
pub(crate) use pluralize::{pluralize, pluralize_gendered};