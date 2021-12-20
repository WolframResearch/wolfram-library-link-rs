macro_rules! unsafe_ref_cast {
    (
        $( #[$attrs:meta] )*
        $vis:vis struct
        $Name:ident<$($param:ident = $val:ty),*>($Inner:ty $(, $zsts:ty )*);
    ) => {
        $( #[$attrs] )*
        // PRE_COMMIT: Document repr(transparent)
        #[repr(transparent)]
        $vis struct $Name< $($param = $val,)* >($Inner $(, $zsts)*);

        // Sanity check that $Name and $Inner have the same size and alignment.
        const _: () = assert!(std::mem::size_of::<$Name>() == std::mem::size_of::<$Inner>());
        const _: () = assert!(std::mem::align_of::<$Name>() == std::mem::align_of::<$Inner>());

        impl<$($param),*> $Name<$($param),*> {
            pub(crate) unsafe fn unsafe_ref_cast(inner: &$Inner) -> &Self {
                &*(inner as *const $Inner as *const Self)
            }
        }
    }
}

pub(crate) use unsafe_ref_cast;
