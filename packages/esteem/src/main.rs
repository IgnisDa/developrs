use clap::{app_from_crate, arg, App, AppSettings};
use env_logger::Env;
use esteem::{
    get_all_project_names, perform_add, perform_init, perform_install_isolated,
    perform_remove, perform_workspace_add, perform_workspace_remove,
};

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
const SKIP: &str = "skip";
const PROJECTS: &str = "PROJECTS";

fn main() -> Result<(), String> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_target(true)
        .init();

    let project_names = get_all_project_names();
    let project_names = &project_names
        .iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();

    let add_subcommand = App::new(ADD_COMMAND)
        .arg(arg!(-s - -skip).help("Skip calling the NPM package manager"))
        .arg(arg!(-D - -development).help("Add as development dependencies"));

    let deps_arg = arg!(<DEPENDENCIES>)
        .required(true)
        .min_values(1)
        .help("The name(s) of the npm packages");

    let remove_subcommand = App::new(REMOVE_COMMAND).alias("rm");

    let project_name_arg = arg!([PROJECT_NAME])
        .required(true)
        .help("The name of the project to make the changes in")
        .possible_values(project_names);

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
                .possible_values(project_names),
        );

    let workspace_subcommand = App::new(WORKSPACE_SUBCOMMAND)
        .about("Interact with workspace scoped dependencies")
        .subcommand(
            add_subcommand
                .clone()
                .arg(deps_arg.clone())
                .about("Install dependencies to a workspace"),
        )
        .subcommand(
            remove_subcommand
                .clone()
                .arg(deps_arg.clone())
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
                .arg(&deps_arg)
                .about("Installs dependencies to a project"),
        )
        .subcommand(
            remove_subcommand
                .clone()
                .arg(&project_name_arg)
                .arg(&deps_arg)
                .about("Removes dependencies from a project (alias: rm)"),
        )
        .subcommand(install_isolated_subcommand)
        .subcommand(workspace_subcommand)
        .get_matches();

    match matches.subcommand() {
        Some((INIT_COMMAND, _)) => {
            perform_init();
        }
        Some((ADD_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of(PROJECT_NAME).unwrap();
            let to_add = sub_matches
                .values_of(DEPENDENCIES)
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            let is_development = sub_matches.is_present(DEVELOPMENT);
            let skip_package_manager = sub_matches.is_present(SKIP);
            trace!("Project Name: {:?}", project_name);
            trace!("Dependencies to add: {:?}", to_add);
            trace!("Development: {:?}", is_development);
            trace!("Calling package manager: {:?}", !skip_package_manager);
            perform_add(
                project_name.into(),
                to_add,
                is_development,
                skip_package_manager,
            );
        }
        Some((REMOVE_COMMAND, sub_matches)) => {
            let project_name = sub_matches.value_of(PROJECT_NAME).unwrap();
            let to_remove = sub_matches
                .values_of(DEPENDENCIES)
                .unwrap()
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            trace!("Project Name: {:?}", project_name);
            trace!("Dependencies to add: {:?}", to_remove);
            perform_remove(project_name.into(), to_remove);
        }
        Some((INSTALL_ISOLATED_COMMAND, sub_matches)) => {
            let project_names = sub_matches
                .values_of(PROJECTS)
                .unwrap()
                .map(String::from)
                .collect::<Vec<String>>();
            trace!("Target projects: {:?}", project_names);
            perform_install_isolated(project_names)
        }
        Some((WORKSPACE_SUBCOMMAND, matches)) => match matches.subcommand() {
            Some((ADD_COMMAND, sub_matches)) => {
                let to_add = sub_matches
                    .values_of(DEPENDENCIES)
                    .unwrap()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                let is_development = sub_matches.is_present(DEVELOPMENT);
                let skip_package_manager = sub_matches.is_present(SKIP);
                trace!("Dependencies to add: {:?}", to_add);
                trace!("Development: {:?}", is_development);
                trace!("Calling package manager: {:?}", !skip_package_manager);
                perform_workspace_add(to_add, is_development, skip_package_manager);
            }
            Some((REMOVE_COMMAND, sub_matches)) => {
                let to_remove = sub_matches
                    .values_of(DEPENDENCIES)
                    .unwrap()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                trace!("Dependencies to remove: {:?}", to_remove);
                perform_workspace_remove(to_remove);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
