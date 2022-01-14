use clap::{app_from_crate, arg, App, AppSettings};
use env_logger::Env;
use esteem::{
    get_project_files_for_all_projects, perform_add, perform_init,
    perform_install_isolated, perform_remove, Workspace, WORKSPACE_FILE,
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

    let all_projects: HashMap<String, PathBuf>;

    match workspace {
        Ok(data) => {
            all_projects = get_project_files_for_all_projects(&data.projects);
        }
        Err(_) => {
            warn!("This project does not have a {:?} file. The commands will not work as expected. Are you running esteem in the correct directory?", WORKSPACE_FILE);
            all_projects = HashMap::new();
        }
    }

    let project_names: Vec<&str> = all_projects.keys().map(|f| f.as_str()).collect();

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
                        .help("The name of the project to which the dependency must be installed")
                        .possible_values(&project_names)
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
                        .possible_values(&project_names)
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
                    "Installs only the dependencies of a particular project.",
                )
                .after_help("NOTE: This mutates `package.json` in place and should be used with care.")
                .arg(
                    arg!([PROJECT_NAME])
                        .required(true)
                        .help("The name of the project whose dependencies should be installed")
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
            let to_add = sub_matches
                .values_of("DEPENDENCIES")
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let is_development = sub_matches.is_present("development");
            let project_path = all_projects.get(project_name).unwrap().clone();
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_add);
            trace!("Development: {:?}", is_development);
            perform_add(project_path, is_development, to_add);
        }
        Some((REMOVE_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of("PROJECT_NAME").unwrap();
            let to_remove = sub_matches
                .values_of("DEPENDENCIES")
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let project_path = all_projects.get(project_name).unwrap().clone();
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_remove);
            perform_remove(project_path, to_remove, all_projects);
        }
        Some((INSTALL_ISOLATED_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of("PROJECT_NAME").unwrap();
            let project_path = all_projects.get(project_name).unwrap().clone();
            perform_install_isolated(project_path)
        }
        _ => unreachable!(),
    }

    Ok(())
}
