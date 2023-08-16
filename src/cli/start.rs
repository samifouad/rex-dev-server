use crate::cli::Args;

pub async fn cmd (context: Args) {

    // defaults
    let mut init_watch = false;

    // override defaults if param provided
    let init_port = context.params.get("--port").cloned().unwrap_or("9001".to_string());
    let init_mode = context.params.get("--mode").cloned().unwrap_or("dev".to_string());
    let init_folder = context.params.get("--folder").cloned().unwrap_or("./app".to_string());

    if let Some (folder) = context.flags.get("--watch") {
        init_watch = true;
    }

    if init_watch {
        // separate scoped thread to watch the filesystem & communicate with the main thread's http server
        // not sure if clone() is ideal solution here, but works for now
        let folder_clone = init_folder.clone();
        let port_clone = init_port.clone();
        std::thread::spawn(move || {
            crate::watch::writes(folder_clone.as_str(), port_clone.as_str());
        });
    }

    let app_data = crate::harvest::app_folder(init_folder.as_str()).unwrap();

    println!("appRoot: {:?}", app_data.app_root_files);
    println!("Pages: {:?}", app_data.pages_files);
    println!("API routes: {:?}", app_data.api_files);

    let converted_paths: Vec<Option<(String, String)>> = app_data.pages_files
        .iter()
        .map(|path| convert_path(path.as_str()))
        .collect();

    for option in converted_paths {
        if let Some((input, output)) = option {
            println!("{} -> {}", input, output);
        }
    }


    let converted_js_paths: Vec<Option<(String, String)>> = app_data.api_files
        .iter()
        .map(|jspath| convert_js_path(jspath.as_str()))
        .collect();

    for option in converted_js_paths {
        if let Some((input, output)) = option {
            println!("{} -> {}", input, output);
        }
    }

     //setup server on main thread
     let mut app = crate::http::Http::new();

     // define routes
     app.html("/index.html", "/dist/index.html");
     app.html("/favicon.ico", "/dist/favicon.ico");
     app.html("/test", "/dist/404.html");
 
     let routes = app.html_routes;
 
     // handle requests
     crate::http::start(routes).await
}

fn convert_path(input_path: &str) -> Option<(String, String)> {
    // Check if path starts with "/api"
    if input_path.starts_with("api") || input_path.starts_with("/api") {
        return None;
    }

    // Split the input path into segments
    let segments: Vec<&str> = input_path.split('/').collect();

    // Ensure there is at least one segment
    if segments.is_empty() {
        return None;
    }

    // Get the last segment (file name)
    let last_segment = segments.last().unwrap();

    // Remove the ".html" extension from the last segment
    let file_name_without_extension = if last_segment.ends_with(".html") {
        &last_segment[..last_segment.len() - 5] // Remove ".html"
    } else {
        last_segment
    };

    // Construct the route for the current input
    let route = if file_name_without_extension == "index" {
        // If it's an "index.html" file at the root, return root path
        if segments.len() == 1 {
            "".to_string()
        } else {
            // Return path excluding the "index" segment
            let constructed_route = segments[..segments.len()-1].join("/");
            if constructed_route.starts_with("/") {
                constructed_route
            } else {
                format!("/{}", constructed_route)
            }
        }
    } else {
        let constructed_route = segments[..segments.len()-1].join("/") + "/" + file_name_without_extension;
        if constructed_route.starts_with("/") {
            constructed_route
        } else {
            format!("/{}", constructed_route)
        }
    };
    
    Some((input_path.to_string(), route))
}

fn convert_js_path(input_path: &str) -> Option<(String, String)> {
    // Split the input path into segments
    let segments: Vec<&str> = input_path.split('/').collect();

    // Ensure there is at least one segment
    if segments.is_empty() {
        return None;
    }

    // Get the last segment (file name)
    let last_segment = segments.last().unwrap();

    // Remove the ".js" extension from the last segment
    if !last_segment.ends_with(".js") {
        return None; // Skip files that aren't .js
    }
    let file_name_without_extension = &last_segment[..last_segment.len() - 3];

    let mut constructed_route = segments[..segments.len()-1].join("/") + "/" + file_name_without_extension;

    // Handle starting double slashes
    if constructed_route.starts_with("//") {
        constructed_route.remove(0);
    }

    // Prepend "/api" to the route
    Some((input_path.to_string(), format!("/api{}", constructed_route)))
}
