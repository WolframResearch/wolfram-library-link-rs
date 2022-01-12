use std::{collections::HashMap, os::raw::c_int, sync::Mutex};

use once_cell::sync::Lazy;

use wolfram_library_link::{
    self as wll,
    expr::{Expr, ExprKind, Symbol},
    managed::{Id, ManagedExpressionEvent},
    sys,
};

wll::export_wstp![
    set_instance_value(_);
    get_instance_data(_);
];

wll::generate_loader![load_managed_exprs_functions];

/// Storage for all instances of [`MyObject`] associated with managed expressions
/// created using `CreateManagedLibraryExpression`.
static INSTANCES: Lazy<Mutex<HashMap<Id, MyObject>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Clone)]
struct MyObject {
    value: String,
}

#[no_mangle]
pub unsafe extern "C" fn WolframLibrary_initialize(
    lib: sys::WolframLibraryData,
) -> c_int {
    if let Err(()) = wll::initialize(lib) {
        return 1;
    }

    // Register `manage_instance()` as the handler for managed expressions created using:
    //
    //     CreateManagedLibraryExpression["my_object", _]
    wll::managed::register_library_expression_manager("my_object", manage_instance);

    return 0;
}

fn manage_instance(action: ManagedExpressionEvent) {
    let mut instances = INSTANCES.lock().unwrap();

    match action {
        ManagedExpressionEvent::Create(id) => {
            // Insert a new MyObject instance with some default values.
            instances.insert(id, MyObject {
                value: String::from("default"),
            });
        },
        ManagedExpressionEvent::Drop(id) => {
            if let Some(obj) = instances.remove(&id) {
                drop(obj);
            }
        },
    }
}

/// Set the `MyObject.value` field for the specified instance ID.
fn set_instance_value(args: Vec<Expr>) {
    assert!(args.len() == 2, "set_instance_value: expected 2 arguments");

    let id: u32 = unwrap_id_arg(&args[0]);
    let value: String = match args[1].kind() {
        ExprKind::String(str) => str.clone(),
        _ => panic!("expected 2nd argument to be a String, got: {}", args[1]),
    };

    let mut instances = INSTANCES.lock().unwrap();

    let instance: &mut MyObject =
        instances.get_mut(&id).expect("instance does not exist");

    instance.value = value;
}

/// Get the fields of the `MyObject` instance for the specified instance ID.
fn get_instance_data(args: Vec<Expr>) -> Expr {
    assert!(args.len() == 1, "get_instance_data: expected 1 argument");

    let id: u32 = unwrap_id_arg(&args[0]);

    let MyObject { value } = {
        let instances = INSTANCES.lock().unwrap();

        instances
            .get(&id)
            .cloned()
            .expect("instance does not exist")
    };

    Expr::normal(Symbol::new("System`Association"), vec![Expr::normal(
        Symbol::new("System`Rule"),
        vec![Expr::string("Value"), Expr::string(value)],
    )])
}

fn unwrap_id_arg(arg: &Expr) -> u32 {
    match arg.kind() {
        ExprKind::Integer(int) => u32::try_from(*int).expect("id overflows u32"),
        _ => panic!("expected Integer instance ID argument, got: {}", arg),
    }
}
