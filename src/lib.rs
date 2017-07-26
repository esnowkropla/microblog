extern crate handlebars_iron;

use handlebars_iron::{HandlebarsEngine, DirectorySource};

pub fn setup_handlebars() -> HandlebarsEngine {
    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./templates/", ".hbs")));

    if let Err(r) = hbse.reload() {
        panic!("{}", r.cause);
    }

    return hbse;
}
