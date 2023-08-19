use crate::cli::Args;

pub fn cmd(context: &Args) {
    // defaults
    let mut init_folder = "./app";

    if let Some(folder) = context.params.get("--folder") {
        init_folder = folder;
    }

    let app_data = crate::harvest::app_folder(init_folder).unwrap();

    println!("appRoot: {:?}", app_data.app_root_files);
    println!("Pages: {:?}", app_data.pages_files);
    println!("API routes: {:?}", app_data.api_files);
}
