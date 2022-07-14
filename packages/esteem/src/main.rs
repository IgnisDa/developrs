use clap::{app_from_crate, arg, App, AppSettings};
use env_logger::Env;
use esteem::{
    perform_add, perform_init, perform_install_isolated, perform_remove,
    perform_workspace_add, EsteemWorkspace, WORKSPACE_FILE,
};
use std::{collections::BTreeMap, path::PathBuf};

#[macro_use]
extern crate log;

const ADD_COMMAND: &str = "add";
const REMOVE_COMMAND: &str = "remove";
const INSTALL_ISOLATED_COMMAND: &str = "install-isolated";
const INIT_COMMAND: &str = "init";
const WORKSPACE_SUBCOMMAND: &str = "workspace";
const PROJECT_NAME: &str = "PROJECT_NAME";
const DEPENDENCIES: &str = "DEPENDENCIES";
const DEVELOPMENT: &str = "development";

fn main() -> Result<(), String> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    let workspace = EsteemWorkspace::from_current_directory();

    let all_projects = match workspace {
        Ok(data) => data.projects,
        Err(_) => {
            warn!("This project does not have a {:?} file. The commands will not work as expected. Are you running esteem in the correct directory?", WORKSPACE_FILE);
            BTreeMap::new()
        }
    };

    let mut project_names: Vec<&str> = all_projects.keys().map(|f| f.as_str()).collect();
    project_names.sort();

    let add_subcommand = App::new(ADD_COMMAND)
        .arg(
            arg!(<DEPENDENCIES>)
                .required(true)
                .min_values(1)
                .help("The name(s) of the npm packages to install"),
        )
        .arg(arg!(-D - -development).help("Add as development dependencies"));

    let remove_subcommand = App::new(REMOVE_COMMAND).alias("rm").arg(
        arg!(<DEPENDENCIES>)
            .required(true)
            .min_values(1)
            .help("The name of the npm packages to remove"),
    );

    let project_name_arg = arg!([PROJECT_NAME])
        .required(true)
        .help("The name of the project to make the changes in")
        .possible_values(&project_names);

    let init_subcommand =
        App::new(INIT_COMMAND).about("Initializes the project to be used with esteem");

    let install_isolated_subcommand = App::new(INSTALL_ISOLATED_COMMAND)
        .about("Isolate only dependencies of a few projects")
        .after_help(
            "NOTE: This mutates `package.json` in place and should be used with care.",
        )
        .arg(
            arg!([PROJECTS])
                .required(true)
                .min_values(1)
                .help("The names of the projects whose dependencies should be installed")
                .possible_values(&project_names),
        );

    let workspace_subcommand = App::new(WORKSPACE_SUBCOMMAND)
        .about("Interact with workspace scoped dependencies")
        .subcommand(
            add_subcommand
                .clone()
                .about("Install dependencies to a workspace"),
        )
        .subcommand(
            remove_subcommand
                .clone()
                .about("Removes dependencies from a workspace (alias: rm)"),
        );

    let matches = app_from_crate!()
        .global_setting(AppSettings::PropagateVersion)
        .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(init_subcommand)
        .subcommand(
            add_subcommand
                .clone()
                .arg(&project_name_arg)
                .about("Installs dependencies to a project"),
        )
        .subcommand(
            remove_subcommand
                .clone()
                .arg(&project_name_arg)
                .about("Removes dependencies from a project (alias: rm)"),
        )
        .subcommand(install_isolated_subcommand)
        .subcommand(workspace_subcommand)
        .get_matches();

    match matches.subcommand() {
        Some((INIT_COMMAND, _)) => {
            perform_init(all_projects.clone());
        }
        Some((ADD_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of(PROJECT_NAME).unwrap();
            let project_path = all_projects.get(project_name).cloned().unwrap();
            let to_add = sub_matches
                .values_of(DEPENDENCIES)
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let is_development = sub_matches.is_present(DEVELOPMENT);
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_add);
            trace!("Development: {:?}", is_development);
            perform_add(project_path, is_development, to_add);
        }
        Some((REMOVE_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of(PROJECT_NAME).unwrap();
            let mut is_global = false;
            let mut project_path = None;
            if project_name == "workspace" {
                is_global = true;
                project_path = all_projects.get(project_name).cloned();
            }
            let to_remove = sub_matches
                .values_of(DEPENDENCIES)
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            trace!("Project Name: {:?}", project_name);
            trace!("Project path: {:?}", project_path);
            trace!("Dependencies to add: {:?}", to_remove);
            perform_remove(project_path, to_remove, all_projects.clone(), is_global);
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
        Some((WORKSPACE_SUBCOMMAND, matches)) => match matches.subcommand() {
            Some((ADD_COMMAND, sub_matches)) => {
                let to_add = sub_matches
                    .values_of(DEPENDENCIES)
                    .unwrap()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                let is_development = sub_matches.is_present(DEVELOPMENT);
                perform_workspace_add(is_development, to_add);
            }
            Some((REMOVE_COMMAND, _sub_matches)) => {
                todo!()
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
