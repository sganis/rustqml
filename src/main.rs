use cstr::cstr;
use qmetaobject::prelude::*;

mod todo;

qrc!(my_resource,
    "todos/qml" {
        "main.qml",
    },
);

fn main() {
    my_resource();
    qml_register_type::<todo::Todos>(cstr!("RustCode"), 1, 0, cstr!("Todos"));
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/todos/qml/main.qml".into());
    engine.exec();
}
