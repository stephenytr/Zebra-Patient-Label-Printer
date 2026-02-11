use chrono::{Local, NaiveDate};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

const ZEBRA_VENDOR_ID: &str = "0a5f";

#[derive(Debug)]
struct PrinterDevice {
    device_path: PathBuf,
    vendor_id: String,
}

#[derive(Debug)]
struct Data {
    first_name: String,
    last_name: String,
    dob: String,
    gender: String,
    current_datetime: String,
    barcode_bool: bool,
    barcode: String
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn format_date_ddmmyyyy(input: &str) -> Option<String> {
    let cleaned: String = input.chars().filter(|c| c.is_numeric()).collect();
    
    if cleaned.len() == 8 {
        let day = &cleaned[0..2];
        let month = &cleaned[2..4];
        let year = &cleaned[4..8];

        if let Ok(_parsed_date) =
            NaiveDate::parse_from_str(&format!("{}-{}-{}", year, month, day), "%Y-%m-%d")
        {
            return Some(format!("{}/{}/{}", day, month, year));
        }
    }

    None
}

fn read_sysfs_file(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn detect_printers() -> Vec<PrinterDevice> {
    let mut printers = Vec::new();

    let usbmisc_path = Path::new("/sys/class/usbmisc");

    if !usbmisc_path.exists() {
        return printers;
    }

    if let Ok(entries) = fs::read_dir(usbmisc_path) {
        for entry in entries.flatten() {
            let device_name = entry.file_name().to_string_lossy().to_string();

            if !device_name.starts_with("lp") {
                continue;
            }

            let device_link = entry.path().join("device");
            if let Ok(usb_interface_path) = fs::read_link(&device_link) {
                let usb_device_path = entry.path().join(&usb_interface_path).join("..");

                if let Ok(canonical) = fs::canonicalize(&usb_device_path) {
                    let vendor_id = read_sysfs_file(&canonical.join("idVendor"));
                    let product_id = read_sysfs_file(&canonical.join("idProduct"));

                    if let (Some(vid), Some(_pid)) = (vendor_id, product_id) {
                        let device_path = PathBuf::from(format!("/dev/usb/{}", device_name));

                        printers.push(PrinterDevice {
                            device_path,
                            vendor_id: vid,
                        });
                    }
                }
            }
        }
    }

    printers
}

fn find_zebra_printer() -> Option<PathBuf> {
    detect_printers()
        .into_iter()
        .find(|p| p.vendor_id == ZEBRA_VENDOR_ID)
        .map(|p| p.device_path)
}

fn generate_zpl(data: &Data) -> String {
    if data.barcode_bool {
        format!(
            r#"^XA
    ^FO40,30^A0N,25,25^FD{}, {}^FS
    ^FO40,55^A0N,25,25^FDDOB: {}, {}^FS
    ^FO40,80^A0N,25,25^FDDate: {}^FS
    ^FO40,105^BY3^BCN,70,Y,N,N,A^FD{}^FS
    ^XZ"#,
            data.last_name.to_uppercase(), 
            data.first_name.to_uppercase(), 
            data.dob, 
            data.gender.to_uppercase(), 
            data.current_datetime,
            data.barcode
        )
    } else {
        format!(
            r#"^XA
    ^FO40,30^A0N,25,25^FD{}, {}^FS
    ^FO40,55^A0N,25,25^FDDOB: {}, {}^FS
    ^FO40,80^A0N,25,25^FDDate: {}^FS
    ^XZ"#,
            data.last_name.to_uppercase(), 
            data.first_name.to_uppercase(), 
            data.dob, 
            data.gender.to_uppercase(), 
            data.current_datetime
        )
    }
}

fn print_label(zpl: &str, printer_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).open(printer_path)?;

    file.write_all(zpl.as_bytes())?;
    file.flush()?;

    Ok(())
}

fn main() {
    println!("=== Label Printer ===");

    let detected_printer = find_zebra_printer();
    let default_path = match &detected_printer {
        Some(path) => {
            println!("✓ Detected Zebra printer at: {}", path.display());
            path.to_string_lossy().to_string()
        }
        None => {
            println!("⚠ No Zebra printer detected, using default path");
            "/dev/usb/lp0".to_string()
        }
    };

    let printer_path = default_path;

    println!("\n--- New Label ---");
    let mut data: Data = Data { 
        first_name: read_input("First name: "),
        last_name: read_input("Last name: "), 
        dob: loop {
            let input = read_input("Date of birth (DDMMYYYY or DD/MM/YYYY): ");
            if let Some(formatted) = format_date_ddmmyyyy(&input) {
                break formatted;
            } else {
                println!("Invalid date format. Please use DDMMYYYY or DD/MM/YYYY");
            }
        }, 
        gender: read_input("Gender: "), 
        current_datetime: Local::now().format("%d/%m/%Y, %H:%M").to_string(), 
        barcode_bool: matches!(read_input("Do you want to print a barcode? (Y/N): ").as_str(), "Y" | "y"), 
        barcode: "0".to_string()
    };

    if data.barcode_bool {
        data.barcode = read_input("Please enter a barcode: ");
    }

    let num_labels = read_input("Num Labels: ");

    let number: i32 = num_labels.trim().parse().expect("Please enter a valid integer");

    let zpl = generate_zpl(&data);
    
    let mut unsuccsessful: bool = false;

    loop {
        for _x in 0..number {
            match print_label(&zpl, &printer_path) {
                Ok(_) => {
                    println!("\n✓ Label sent to printer successfully!");
                    unsuccsessful = false;
                    continue;
                }
                Err(e) => {
                    eprintln!("\n✗ Error printing label: {}", e);
                    eprintln!("Make sure the printer is connected at: {}", printer_path);
                    unsuccsessful = true;
                    break;
                }
            }
        }
        if unsuccsessful {
            let try_again = read_input("Try again? (Y/N): ");
            match try_again.as_str() {
                "y" | "Y" | "yes" => continue,
                _ => break,
                
            }
        } else {
            break
        }
    }
}       

