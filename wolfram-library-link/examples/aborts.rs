use wolfram_library_link as wll;

wll::export![wait_for_abort()];

/// This function will execute forever until a Wolfram Language abort occurs.
fn wait_for_abort() {
    loop {
        if wll::aborted() {
            return;
        }

        std::thread::yield_now();
    }
}
