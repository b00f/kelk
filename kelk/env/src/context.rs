//! The context for running contract actor

/// `Context` owns the `ContextAPI` reference.
pub struct Context<'a> {
    /// The instance of Blockchain APIs
    pub blockchain: &'a dyn crate::blockchain::Blockchain,

    /// The instance of storage APIs
    pub storage: &'a dyn crate::storage::Storage,
}
