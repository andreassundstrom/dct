use dicom::dicom_value;
use dicom::object::{open_file, Tag};
use dicom::{core::DataElement, dictionary_std::tags};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new randomized dicom from a given input file
    randomize {
        /// Filename for input
        #[arg(short, long)]
        filename: String,
    },
}

fn main() {
    println!("HELLO");
    let args = Cli::parse();
    match &args.command {
        Some(Commands::randomize { filename }) => {
            println!("Randomizing new dicom from file '{}'", filename);
            let _ = randomize_dicom(filename);
        }
        None => {
            println!("No command selected");
        }
    }
}

fn randomize_dicom(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut obj = open_file(input_file).unwrap();

    let new_patient_name = DataElement::new(
        tags::PATIENT_NAME,
        dicom::core::VR::PN,
        dicom_value!(Str, "Andreas"),
    );
    obj.put(new_patient_name);

    let new_filename = format!("{}.random.dcm", input_file);
    println!("Saving new random file '{}'", new_filename);

    obj.write_to_file(new_filename).unwrap();

    Ok(())
}
