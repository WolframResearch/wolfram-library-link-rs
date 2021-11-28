use std::{
    ffi::CStr,
    fs,
    os::raw::{c_char, c_uint, c_void},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use wl_library_link::{
    self as wll,
    sys::{self, mint, MArgument, LIBRARY_FUNCTION_ERROR, LIBRARY_NO_ERROR},
};

struct FileWatcherArgs {
    pause_interval_ms: u64,
    path: PathBuf,
}

/// Start an asynchronous task that will watch for modifications to a file.
///
/// See `RustLink/Tests/AsyncExamples.wlt` for example usage of this function.
#[no_mangle]
pub extern "C" fn start_file_watcher(
    lib_data: sys::WolframLibraryData,
    arg_count: mint,
    args: *mut MArgument,
    res: MArgument,
) -> c_uint {
    let args = unsafe { std::slice::from_raw_parts(args, arg_count as usize) };

    if args.len() != 2 {
        return LIBRARY_FUNCTION_ERROR;
    }

    if wll::initialize(lib_data).is_err() {
        return LIBRARY_FUNCTION_ERROR;
    }

    let task_arg = unsafe {
        FileWatcherArgs {
            pause_interval_ms: u64::try_from(*args[0].integer)
                .expect("i64 interval overflows u64"),
            path: {
                let cstr = CStr::from_ptr(*args[1].utf8string);
                match cstr.to_str() {
                    Ok(s) => PathBuf::from(s),
                    Err(_) => return LIBRARY_FUNCTION_ERROR,
                }
            },
        }
    };

    let create_async_task_with_thread: unsafe extern "C" fn(
        Option<unsafe extern "C" fn(i64, *mut c_void)>,
        *mut c_void,
    ) -> sys::mint = unsafe {
        (*wll::get_library_data().ioLibraryFunctions)
            .createAsynchronousTaskWithThread
            .expect("createAsynchronousTaskWithThread callback is NULL")
    };

    // FIXME: This box is being leaked. Where is an appropriate place to drop it?
    let task_arg = Box::into_raw(Box::new(task_arg)) as *mut c_void;

    // Spawn a new thread, which will run in the background and check for file
    // modifications.
    unsafe {
        let task_id: mint =
            create_async_task_with_thread(Some(file_watch_thread_function), task_arg);
        *res.integer = task_id;
    }

    LIBRARY_NO_ERROR
}

/// This function is called first from the spawned background thread.
extern "C" fn file_watch_thread_function(async_object_id: mint, task_arg: *mut c_void) {
    let task_arg = task_arg as *mut FileWatcherArgs;
    let task_arg: &FileWatcherArgs = unsafe { &*task_arg };

    let FileWatcherArgs {
        pause_interval_ms,
        ref path,
    } = *task_arg;

    let (
        async_task_is_alive,
        create_data_store,
        data_store_add_integer,
        raise_async_event,
    ): (
        unsafe extern "C" fn(mint) -> sys::mbool,
        unsafe extern "C" fn() -> sys::DataStore,
        unsafe extern "C" fn(sys::DataStore, mint),
        unsafe extern "C" fn(mint, *mut c_char, sys::DataStore),
    ) = {
        let io_funcs = unsafe { *wll::get_library_data().ioLibraryFunctions };

        (
            io_funcs
                .asynchronousTaskAliveQ
                .expect("asynchronousTaskAliveQ callback is NULL"),
            io_funcs
                .createDataStore
                .expect("createDataStore callback is NULL"),
            io_funcs
                .DataStore_addInteger
                .expect("DataStore_addInteger callback is NULL"),
            io_funcs
                .raiseAsyncEvent
                .expect("raiseAsyncEvent callback is NULL"),
        )
    };

    let mut prev_changed: Option<SystemTime> = fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok();

    // Stateful closure which checks if the file at `path` has been modified since the
    // last time this closure was called (and `prev_changed was updated). Using a closure
    // simplifies the control flow in the main `loop` below, which should sleep on every
    // iteration regardless of how this function returns.
    let mut check_for_modification = || -> Option<_> {
        let changed: Option<fs::Metadata> = fs::metadata(path).ok();

        let notify: Option<SystemTime> = match (&prev_changed, changed) {
            (Some(prev), Some(latest)) => {
                let latest: SystemTime = match latest.modified() {
                    Ok(latest) => latest,
                    Err(_) => return None,
                };

                if *prev != latest {
                    prev_changed = Some(latest.clone());
                    Some(latest)
                } else {
                    None
                }
            },
            // TODO: Notify on file removal?
            (Some(_prev), None) => None,
            (None, Some(latest)) => latest.modified().ok(),
            (None, None) => None,
        };

        let time = notify?;

        let since_epoch = match time.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => duration,
            Err(_) => return None,
        };

        let since_epoch = since_epoch.as_secs();

        Some(since_epoch)
    };

    loop {
        if unsafe { async_task_is_alive(async_object_id) } == 0 {
            break;
        }

        // Check to see if the file has been modified. If it has, raise an async event
        // called "change", and attach the modification timestamp as event data.
        if let Some(modification) = check_for_modification() {
            unsafe {
                let data_store: sys::DataStore = create_data_store();
                data_store_add_integer(data_store, modification as i64);

                raise_async_event(
                    async_object_id,
                    "change\0".as_ptr() as *mut _,
                    data_store,
                )
            }
        }

        // Wait for a bit before polling again for any changes to the file.
        std::thread::sleep(Duration::from_millis(pause_interval_ms));
    }
}
