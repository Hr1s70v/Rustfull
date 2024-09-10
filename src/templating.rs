use tera::{Tera, Context};
use std::fs;
use std::path::{Path, PathBuf};

// Function to set up Tera and print registered templates
pub fn setup_tera() -> Tera {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };

    // Debug: Print all registered templates
    println!("Registered templates:");
    for template in tera.get_template_names() {
        println!("{}", template);
    }

    tera
}

fn render_and_write_templates(
    tera: &Tera,
    template_dir: &str,
    output_dir: &Path,
    context: &Context,
) {
    println!("Reading from template directory: {}", template_dir);
    println!("Writing to output directory: {:?}", output_dir);

    // Ensure the output directory exists
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory: {}", e);
        std::process::exit(1);
    }

    if let Ok(entries) = fs::read_dir(template_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(template_dir).unwrap();
                
                // Store the result of to_str() in a variable
                let template_name_str = relative_path
                    .to_str()
                    .unwrap_or_else(|| {
                        eprintln!("Template path is not valid UTF-8: {}", relative_path.display());
                        ""
                    });

                // Replace path separators and get the template name
                let template_name_with_extension = template_name_str
                    .replace("\\", "/"); // Ensure the path separator is correct
                let template_name = template_name_with_extension
                    .strip_suffix(".tera")
                    .unwrap_or_else(|| {
                        eprintln!("Template file has no '.tera' extension: {}", relative_path.display());
                        ""
                    });

                println!("Found template file: {}", relative_path.display());

                let output_path = output_dir.join(relative_path);
                if let Some(parent) = output_path.parent() {
                    if let Err(e) = fs::create_dir_all(parent) {
                        eprintln!("Failed to create directory: {}", e);
                        std::process::exit(1);
                    }
                }

                println!("Rendering template: {}", template_name);
                match tera.render(template_name, context) {
                    Ok(rendered) => {
                        if let Err(e) = fs::write(output_path, rendered) {
                            eprintln!("Failed to write file: {}", e);
                            std::process::exit(1);
                        }
                        println!("Successfully rendered template: {}", template_name);
                    }
                    Err(e) => {
                        eprintln!("Failed to render template '{}': {}", template_name, e);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read template directory: {}", template_dir);
        std::process::exit(1);
    }
}






// Main function to handle template rendering
pub fn template_rendering(project_name: &str, frontend_framework: &str, backend_framework: &str) {
    // Initialize Tera and print registered templates
    let tera = setup_tera();

    // Create the context and insert the variables
    let mut context = Context::new();
    context.insert("project_name", project_name);
    context.insert("frontend_framework", frontend_framework);
    context.insert("backend_framework", backend_framework);

    // Define the output directory as Path
    let output_dir = Path::new(project_name);
    let output_path = output_dir.to_path_buf();

    // Ensure the output directory exists
    if let Err(e) = fs::create_dir_all(&output_path) {
        eprintln!("Failed to create output directory: {}", e);
        std::process::exit(1);
    }

    // Define the frontend and backend directories
    let frontend_dir = output_path.join(format!("{}_frontend", project_name));
    let backend_dir = output_path.join(format!("{}_backend", project_name));

    if let Err(e) = fs::create_dir_all(&frontend_dir) {
        eprintln!("Failed to create frontend directory: {}", e);
        std::process::exit(1);
    }

    if let Err(e) = fs::create_dir_all(&backend_dir) {
        eprintln!("Failed to create backend directory: {}", e);
        std::process::exit(1);
    }

    // Define the frontend and backend template directories
    let frontend_template_dir = format!("templates/frontend/{}/", frontend_framework.to_lowercase());
    let backend_template_dir = format!("templates/backend/{}/", backend_framework.to_lowercase());

    println!("Frontend template directory: {}", frontend_template_dir);
    println!("Backend template directory: {}", backend_template_dir);

    // Render and write templates for frontend
    render_and_write_templates(
        &tera,
        &frontend_template_dir,
        &frontend_dir,
        &context,
    );

    // Render and write templates for backend
    render_and_write_templates(
        &tera,
        &backend_template_dir,
        &backend_dir,
        &context,
    );

    println!("Templates generated successfully in {}", output_path.display());
}
