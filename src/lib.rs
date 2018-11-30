#[macro_export]
macro_rules! link_wrapper {
    (fn $name:ident($lib_data:ident, $args:ident, $res:ident) -> u32 $body:block) => {
        #[no_mangle]
        #[allow(non_snake_case)]
        pub unsafe fn $name($lib_data: WolframLibraryData, arg_count: mint,
                            $args: *const MArgument, $res: MArgument) -> u32 {
            let arg_count = match usize::try_from(arg_count) {
                Ok(count) => count,
                // NOTE: This will never happen as long as LibraryLink doesn't give us a
                //       negative argument count. If that happens, something else has
                //       gone seriously wrong, so let's do the least unhelpful thing.
                // TODO: Is there a better error we could return here?
                Err(_) => return LIBRARY_FUNCTION_ERROR,
            };
            let $args: &[MArgument] = ::std::slice::from_raw_parts($args, arg_count);
            $body
        }
    }
}
