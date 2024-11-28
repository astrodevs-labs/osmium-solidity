use clap::{arg, Parser};
use solidhunter::aggregate::aggregate_diags;
use solidhunter::errors::SolidHunterError;
use solidhunter::linter::SolidLinter;
use solidhunter::rules::rule_impl::create_rules_file;
use solidhunter::types::{FileDiags, LintResult};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Paths to the projects to lint")]
    paths: Vec<String>,

    #[arg(
        short = 'r',
        long = "rules",
        default_value = ".solidhunter.json",
        help = "Specify rules file"
    )]
    rules_file: String,

    #[arg(
        short = 'j',
        long = "json",
        default_value = "false",
        help = "Outputs a json format instead"
    )]
    to_json: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        default_value = "false",
        help = "Verbose output"
    )]
    verbose: bool,

    #[arg(
        short = 'i',
        long = "init",
        default_value = "false",
        help = "Initialize rules file"
    )]
    init: bool,

    #[arg(short = 'e', long = "exclude", help = "Specify excluded files")]
    exclude: Option<Vec<String>>,

    #[arg(
        short = 'd',
        long = "documentation",
        default_value = "false",
        help = "exposes rules documentation"
    )]
    documentation: bool,
}

fn print_result(results: &Vec<LintResult>) {
    for result in results {
        match result {
            Ok(diags) => {
                let aggregated = aggregate_diags(diags.diags.clone());
                let new_file = FileDiags::new(diags.source_file_content.clone(), aggregated);
                print!("{}", &new_file);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() -> Result<(), SolidHunterError> {
    let args = Args::parse();

    if args.documentation {
        let linter: SolidLinter = SolidLinter::new_fileless();

        let json = serde_json::to_string_pretty(&linter.get_documentation());
        match json {
            Ok(j) => {
                println!("{}", j);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        return Ok(());
    }

    if !args.to_json {
        println!();
        println!("SolidHunter: Fast and efficient Solidity linter");
        println!(
            "By {} - v{} - GNU GPL v3",
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_VERSION")
        );
        println!();
    }

    if args.verbose {
        println!("Verbose output enabled");
        println!("Project path: {:?}", args.paths);
        println!("Using rules file: {}", args.rules_file);
        println!("Verbose output: {}", args.verbose);
        println!("Excluded files: {:?}", args.exclude);
        println!("Documentation output: {}", args.documentation);
    }

    if args.init {
        println!("Initializing rules file...");
        if args.paths.is_empty() {
            create_rules_file(&args.rules_file);
        }
        for path in &args.paths {
            if let Ok(metadata) = std::fs::metadata(path.as_str()) {
                if metadata.is_dir() {
                    create_rules_file(&(path.as_str().to_owned() + "/" + args.rules_file.as_str()));
                }
            }
        }
        println!("Done!");
        return Ok(());
    }

    let mut linter: SolidLinter = SolidLinter::new();
    if !args.paths.is_empty() {
        linter.initialize_rules(
            &(args.paths[0].as_str().to_owned() + "/" + args.rules_file.as_str()),
        )?;
    } else {
        linter.initialize_rules(&args.rules_file)?;
    }
    linter.initialize_excluded_files(args.exclude.as_ref(), &args.paths)?;

    let mut results = vec![];
    for path in &args.paths {
        let result = linter.parse_path(path);
        results.push(result);
    }
    // If no path is specified, we use the current directory
    if args.paths.is_empty() {
        let result = linter.parse_path(".");
        results.push(result);
    }
    for (index, path_result) in results.iter().enumerate() {
        if !args.to_json {
            print_result(path_result);
            if index == results.len() - 1 {
                println!();
            }
        } else {
            for res in path_result {
                match res {
                    Ok(diags) => {
                        let json = serde_json::to_string_pretty(&diags);
                        match json {
                            Ok(j) => {
                                println!("{}", j);
                            }
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }
    Ok(())
}
