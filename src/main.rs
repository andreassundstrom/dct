use clap::{Parser, Subcommand};
use dicom::dicom_value;
use dicom::dump::dump_file;
use dicom::object::{open_file, FileDicomObject, InMemDicomObject};
use dicom::{core::DataElement, dictionary_std::tags};
#[derive(Parser)]
#[command(about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display information about a dicom file
    Dump {
        #[arg(short, long)]
        filename: String,
    },
    /// Create a new randomized dicom from a given input file
    Randomize {
        /// Filename for input
        #[arg(short, long)]
        filename: String,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Randomize { filename } => {
            println!("Randomizing new dicom from file '{}'", filename);
            let _ = randomize_dicom(filename);
        }
        Commands::Dump { filename } => {
            println!("Dumping file '{}'", filename);
            dump_dicom_file(filename);
        }
    }
}

fn dump_dicom_file(input_file: &str) {
    let obj = match open_file(input_file) {
        Ok(obj) => obj,
        Err(err) => panic!("Failed to open file: {:?}", err),
    };

    dump_file(&obj).unwrap();
}

fn randomize_dicom(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut obj = open_file(input_file).unwrap();

    replace_tags(&mut obj);

    let uid = uuid::Uuid::new_v4();

    let new_filename = format!("{}_{}.dcm", input_file.replace(".dcm", ""), uid.to_string());
    println!("Saving new random file '{}'", new_filename);

    obj.write_to_file(new_filename).unwrap();

    Ok(())
}

fn replace_tags(dicom: &mut FileDicomObject<InMemDicomObject>) {
    let new_patient_name = DataElement::new(
        tags::PATIENT_NAME,
        dicom::core::VR::PN,
        dicom_value!(Str, uuid::Uuid::new_v4().to_string()),
    );
    dicom.put(new_patient_name);

    let new_patient_id = DataElement::new(
        tags::PATIENT_ID,
        dicom::core::VR::ST,
        dicom_value!(Str, uuid::Uuid::new_v4().to_string()),
    );
    dicom.put(new_patient_id);
}
