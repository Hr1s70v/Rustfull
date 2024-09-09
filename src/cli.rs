use cliclack::{input, select, multi_progress, confirm, intro, set_theme, Theme, ThemeState};
use std::error::Error;
use colored::*;
use console::{Style, style}; // Ensure this is the correct import for console::Style

struct BlueTheme;

impl Theme for BlueTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().cyan(),
            ThemeState::Error(_) => Style::new().red(),
            _ => Style::new().cyan().dim(),
        }
    }

    fn state_symbol_color(&self, _state: &ThemeState) -> Style {
        Style::new().cyan()
    }

    fn info_symbol(&self) -> String {
        "⚙".into()
    }
}

pub fn user_input() -> Result<(), Box<dyn Error>> {
    set_theme(BlueTheme);
    let multi = multi_progress(" ");
    intro(style(" SETUP ").on_cyan().black())?;
    // Get project name from user
    let project_name: String = input("What will your project be called? (my-rustfull-app)")
        .placeholder("my-rustfull-app")
        .validate(|name: &String| {
            if name.is_empty() {
                Err("Project name cannot be empty.")
            } else if name.contains('/') || name.contains('\\') {
                Err("Project name cannot contain '/' or '\\'.")
            } else {
                Ok(())
            }
        })
        .interact()?;

    // Get selected frontend language
    let frontend = select("Which language will you use for your frontend?")
        .item("ts", "TypeScript", "")
        .item("js", "JavaScript", "")
        .item("rs", "Rust", "")
        .interact()?;

    // Save the language variable for later use
    let language = match frontend {
        "ts" => "TypeScript",
        "js" => "JavaScript",
        "rs" => "Rust",
        _ => return Err("Invalid selection".into()),
    };

    // Handle frontend framework selection
    let frontend_framework = if language == "Rust" {
        let rust_option = select("Which frontend framework would you like to use?")
            .item("Dioxus", "Dioxus", "")
            .item("Yew", "Yew", "")
            .item("Seed", "Seed", "")
            .item("Percy", "Percy", "")
            .interact()?;
        rust_option.to_string() // Convert to String if needed
    } else {
        let js_ts_option = select("Which frontend framework would you like to use?")
            .item("React", "React", "")
            .item("Vue", "Vue", "")
            .item("Svelte", "Svelte", "")
            .interact()?;
        js_ts_option.to_string() // Convert to String if needed
    };

    // Display selected frontend framework

    let backend_framework = select("Which framework will you use for your backend?")
        .item("Actix Web", "Actix Web", "")
        .item("Rocket", "Rocket", "")
        .item("Axum", "Axum", "")
        .item("Warp", "Warp", "")
        .item("Tide", "Tide", "")
        .item("Salvage", "Salvage", "")
        .item("Gotham", "Gotham", "")
        .interact()?;

    // Ask if additional tools or libraries should be enabled
    let enable_additional_tools = select("Would you like to enable additional tools or libraries?")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()?;

    // Initialize a variable to hold additional tools/libraries
    let tools_list: Vec<String> = if enable_additional_tools == "yes" {
        let additional_tools: String = input("Enter the tools or libraries (Separate with commas)")
            .interact()?;
        
        additional_tools
            .split(',')
            .map(|s| s.trim().to_string()) // Convert each &str to String
            .filter(|s| !s.is_empty()) // Remove empty entries if any
            .collect()
    } else {
        vec![] // No tools or libraries selected
    };


    // Handle other binary responses
    let linting_formatting = match select("Enable code linting and formatting?")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()? {
        "yes" => "Enabled",
        "no" => "Disabled",
        _ => "Unknown",
    };

    let git_repo = match select("Initialize a new git repository? (Y/n)")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()? {
        "yes" => "Initialized",
        "no" => "Not Initialized",
        _ => "Unknown",
    };

    let env_vars = match select("Would you like to configure environment variables for your project? (Y/n)")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()? {
        "yes" => "Configured",
        "no" => "Not Configured",
        _ => "Unknown",
    };

    let docker_support = match select("Would you like to include Docker support for containerization? (Y/n)")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()? {
        "yes" => "Included",
        "no" => "Not Included",
        _ => "Unknown",
    };

    let install_command = match select("Would you like us to run cargo install or npm install for you? (Y/n)")
        .item("yes", "Yes", "")
        .item("no", "No", "")
        .interact()? {
        "yes" => "Run Install",
        "no" => "Do Not Run",
        _ => "Unknown",
    };

    // Display collected information with color
    println!("\nProject Setup Summary:");
    println!("Project Name: {}", project_name.bold().blue());
    println!("Frontend Language: {}", language.bold().blue());
    println!("Frontend Framework: {}", frontend_framework.bold().blue());
    println!("Backend Framework: {}", backend_framework.bold().blue());
    println!("Enable code linting and formatting: {}", linting_formatting);
    println!("Initialize a new git repository: {}", git_repo);
    println!("Configure environment variables: {}", env_vars);
    println!("Include Docker support: {}", docker_support);
    println!("Run install command: {}", install_command);
    println!("Additional tools/libraries: {:?}", tools_list);
    
    
    // Handle continue confirmation
    let should_continue = confirm("Do you want to continue?").interact()?;
    if !should_continue {
        println!("Exiting setup.");
        return Ok(()); // Exit if user does not want to continue
    }

    // Stop the progress bar
    multi.stop();

    Ok(())
}
