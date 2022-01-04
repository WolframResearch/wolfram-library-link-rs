use std::panic;

use wolfram_library_link::{
    self as wll,
    expr::{Expr, Symbol},
};

wll::export![
    test_runtime_function_from_main_thread();
    test_runtime_function_from_non_main_thread();
];

fn test_runtime_function_from_main_thread() -> bool {
    let expr = Expr::normal(Symbol::new("System`Plus"), vec![
        Expr::from(2),
        Expr::from(2),
    ]);

    wll::evaluate(&expr) == Expr::from(4)
}

fn test_runtime_function_from_non_main_thread() -> String {
    let child = std::thread::spawn(|| {
        panic::set_hook(Box::new(|_| {
            // Do nothing, just to avoid printing panic message to stderr.
        }));

        let result = panic::catch_unwind(|| {
            wll::evaluate(&Expr::normal(Symbol::new("System`Plus"), vec![
                Expr::from(2),
                Expr::from(2),
            ]))
        });

        // Restore the previous (default) hook.
        let _ = panic::take_hook();

        result
    });

    let result = child.join().unwrap();

    match result {
        Ok(_) => "didn't panic".to_owned(),
        // We expect the thread to panic
        Err(panic) => {
            if let Some(str) = panic.downcast_ref::<&str>() {
                format!("PANIC: {}", str)
            } else if let Some(string) = panic.downcast_ref::<String>() {
                format!("PANIC: {}", string)
            } else {
                "PANIC".to_owned()
            }
        },
    }
}
