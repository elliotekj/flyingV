use super::*;
use tera::Context;

pub fn generate() {
    let mut context = Context::new();

    let site = json!({
        "name": *SITE_NAME,
    });

    context.add("site", &site);

    render(context);
}

fn render(context: Context) {
    match TEMPLATES.render("views/index.html", context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
}
