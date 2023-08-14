#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

pub struct Args {
    pub flags: std::collections::HashMap<String, bool>,
    pub params: std::collections::HashMap<String, String>,
    pub commands: Vec<String>,
}

pub fn collect (args: Vec<String>) -> Args {

    let mut flags = std::collections::HashMap::new();
    let mut params = std::collections::HashMap::new();
    let mut commands = Vec::new();

    let valid_flags = [
                                    "--help", "help", "-H", 
                                    "--version", "version", "-V", 
                                    "--update", "update", "-U", 
                                    "--watch", "watch", "-W"
                                ];

    let valid_params = [
                                    "--port",
                                    "--mode",
                                    "--folder"
                                ];

    let valid_commands = [
                                    "start",
                                    "tree",
                                    "route",
                                    "init",
                                    "build",
                                    "deploy"
                                ];

    let mut iter = args.iter().enumerate();
    while let Some((_i, arg)) = iter.next() {
        if valid_flags.contains(&arg.as_str()) {
            flags.insert(arg.clone(), true);
        } else if valid_params.contains(&arg.as_str()) {
            if let Some(value) = iter.next().map(|(_, v)| v) {
                params.insert(arg.clone(), value.clone());
            } else {
                println!("Error: '{}' requires a value.", arg);
            }
        } else if valid_commands.contains(&arg.as_str()) {
            commands.push(arg.clone());
        }
    };

    Args {
        flags,
        params,
        commands,
    }
}

// provide helpful info if no args are provided
pub fn help () {
    println!("rexds [version {}]\n", env!("CARGO_PKG_VERSION"));
    println!("usage: rexds [--version] [--help] [--update] <command> [<params>]\n");
    println!("rexds expects an ./app folder containing an index.html entrypoint.\n\nyou can override this location with the parameter: --folder <path>\n");
    println!("commands:\n");
    println!("start\t\tstarts the server");
    println!("init\t\tinitialize a new app project");
    println!("tree\t\tdisplay detected routes in tree format");
    println!("route\t\tinspect a route in your project");
    println!("build\t\toptimizes your project for production");
    println!("deploy\t\tdeploys your project to a remote server");
    println!("\n");
}

pub fn version () {
    println!("rexds [version {}]\n\nto check for updates run: rexds --update\n", env!("CARGO_PKG_VERSION"));
}

pub fn update () {
    println!("this will check for updates and offer the ability to run the update. not yet implemented. \n");
}

pub async fn start (context: &Args) {

    // defaults
    let mut init_port = "9001";
    let mut init_folder = "./app";
    let mut init_mode = "dev";

    // override defaults if param provided
    if let Some(port) = context.params.get("--port") {
        init_port = port;
    }

    if let Some(mode) = context.params.get("--mode") {
        init_mode = mode;
    }

    if let Some (folder) = context.params.get("--folder") {
        init_folder = folder;
    }
    
    let app_data = crate::harvest::app_folder(init_folder).unwrap();

    println!("appRoot files: {:?}", app_data.app_root_files);
    println!("Public files: {:?}", app_data.public_files);
    println!("GET files: {:?}", app_data.get_files);
    println!("POST files: {:?}", app_data.post_files);
    println!("PUT files: {:?}", app_data.put_files);

     //setup server on main thread
     let mut app = crate::http::Http::new();

     // define routes
     app.get("/index.html", "/dist/index.html");
     app.get("/favicon.ico", "/dist/favicon.ico");
     app.get("/test", "/dist/404.html");
 
     let routes = app.get_routes;
 
     // handle requests
     crate::http::start(routes).await
}

pub fn tree (context: &Args) {

    // defaults
    let mut init_folder = "./app";

    if let Some (folder) = context.params.get("--folder") {
        init_folder = folder;
    }
    
    let app_data = crate::harvest::app_folder(init_folder).unwrap();

    println!("appRoot files: {:?}", app_data.app_root_files);
    println!("Public files: {:?}", app_data.public_files);
    println!("GET files: {:?}", app_data.get_files);
    println!("POST files: {:?}", app_data.post_files);
    println!("PUT files: {:?}", app_data.put_files);
}

pub fn route (context: &Args) {

    for (key, value) in context.params.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    //println!("inspecting route {}", path.unwrap_or(&"/".to_string()));
}

pub fn init (context: &Args) {

    for (key, value) in context.params.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    //println!("initializing new project folder at {}", path.unwrap_or(&"./app".to_string()));
}

pub fn build (context: &Args) {
    for (key, value) in context.params.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("optimize project for production");
}

pub fn deploy (context: &Args) {

    for (key, value) in context.params.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("deploy project to remote server");
}

pub fn error (msg: Option<&String>) {
    println!("{}", msg.unwrap_or(&"\ninstructions unclear. try '--help' for guidance\n".to_string()));
}