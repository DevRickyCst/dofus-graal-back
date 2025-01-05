use clap::{arg, Command};

/// Arguments CLI
pub struct CliArgs {
    pub mode: String,
    pub table: String,
}

impl CliArgs {
    /// Convertit les arguments parsés en une structure `CliArgs`
    pub fn from_matches(matches: clap::ArgMatches) -> Self {
        let mode = matches.get_one::<String>("mode").unwrap().clone();
        let table = matches.get_one::<String>("table").unwrap().clone();

        CliArgs { mode, table }
    }
}

/// Définit la configuration du CLI
pub fn build_cli() -> Command {
    Command::new("Database CLI")
        .about("Permet de gérer les tables de la base de données")
        .arg(
            arg!(<mode> "Mode d'opération : delete ou update")
                .value_parser(["delete", "update","api_call"]),
        )
        .arg(
            arg!(<table> "Nom de la table à opérer"),
        )
}
