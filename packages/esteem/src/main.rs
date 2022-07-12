use clap::{app_from_crate, arg, App, AppSettings};
use env_logger::Env;
use esteem::{
    get_project_files_for_all_projects, perform_add, perform_init,
    perform_install_isolated, perform_remove, Workspace, WORKSPACE_FILE,
    WORKSPACE_IDENTIFIER,
};
use std::{collections::HashMap, path::PathBuf};

#[macro_use]
extern crate log;

const ADD_COMMAND: &str = "add";
const REMOVE_COMMAND: &str = "remove";
const INSTALL_ISOLATED_COMMAND: &str = "install-isolated";
const INIT_COMMAND: &str = "init";

fn main() -> Result<(), String> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    let workspace = Workspace::new();

    let all_projects = match workspace {
        Ok(data) => get_project_files_for_all_projects(&data.projects),
        Err(_) => {
            warn!("This project does not have a {:?} file. The commands will not work as expected. Are you running esteem in the correct directory?", WORKSPACE_FILE);
            HashMap::new()
        }
    };

    let project_names: Vec<&str> = all_projects.keys().map(|f| f.as_str()).collect();
    let mut project_names_with_workspace = project_names.clone();
    project_names_with_workspace.extend([WORKSPACE_IDENTIFIER]);
    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new(INIT_COMMAND)
                .about("Initializes the project to be used with esteem")
        )
        .subcommand(
            App::new(ADD_COMMAND)
                .about("Installs dependencies to a project")
                .arg(
                    arg!([PROJECT_NAME])
                        .required(true)
                        .help("The name of the project to which the dependency must be installed. If equal to `workspace`, then this dependency will be added globally.")
                        .possible_values(&project_names_with_workspace)
                )
                .arg(
                    arg!(<DEPENDENCIES>)
                        .required(true)
                        .min_values(1)
                        .help("The name(s) of the npm packages to install"),
                )
                .arg(
                    arg!(-D - -development).help("Add as development dependencies")
                )
        )
        .subcommand(
            App::new(REMOVE_COMMAND)
                .about("Removes dependencies from a project (alias: rm)")
                .alias("rm")
                .arg(
                    arg!([PROJECT_NAME])
                        .required(true)
                        .help("The name(s) of the project from which the dependency must be removed")
                        .possible_values(&project_names_with_workspace)
                )
                .arg(
                    arg!(<DEPENDENCIES>)
                        .required(true)
                        .min_values(1)
                        .help("The name of the npm packages to remove"),
                )
        )
        .subcommand(
            App::new(INSTALL_ISOLATED_COMMAND)
                .about(
                    "Isolate only dependencies of a few projects",
                )
                .after_help("NOTE: This mutates `package.json` in place and should be used with care.")
                .arg(
                    arg!([PROJECTS])
                        .required(true)
                        .min_values(1)
                        .help("The names of the projects whose dependencies should be installed")
                        .possible_values(&project_names)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some((INIT_COMMAND, _)) => {
            perform_init(all_projects);
        }
        Some((ADD_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of("PROJECT_NAME").unwrap();
            let mut is_global = false;
            let mut project_path = None;
            if project_name == "workspace" {
                is_global = true;
                project_path = all_projects.get(project_name).cloned();
            }
            let to_add = sub_matches
                .values_of("DEPENDENCIES")
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let is_development = sub_matches.is_present("development");
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_add);
            trace!("Development: {:?}", is_development);
            trace!("Global: {:?}", is_global);
            perform_add(project_path, is_development, to_add, is_global);
        }
        Some((REMOVE_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of("PROJECT_NAME").unwrap();
            let mut is_global = false;
            let mut project_path = None;
            if project_name == "workspace" {
                is_global = true;
                project_path = all_projects.get(project_name).cloned();
            }
            let to_remove = sub_matches
                .values_of("DEPENDENCIES")
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_remove);
            perform_remove(project_path, to_remove, all_projects, is_global);
        }
        Some((INSTALL_ISOLATED_COMMAND, sub_matches)) => {
            let project_names = sub_matches
                .values_of("PROJECTS")
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let project_paths = project_names
                .iter()
                .map(|f| all_projects.get(f).cloned().unwrap())
                .collect::<Vec<PathBuf>>();
            perform_install_isolated(project_paths)
        }
        _ => unreachable!(),
    }

    Ok(())
}
