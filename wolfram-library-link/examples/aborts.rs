use wolfram_library_link as wll;

/// This function will execute forever until a Wolfram Language abort occurs.
#[wll::export]
fn wait_for_abort() {
    loop {
        if wll::aborted() {
            return;
        }

        std::thread::yield_now();
    }
}
