# Label Printer

A command-line Rust application for printing patient identification labels on Zebra label printers using ZPL (Zebra Programming Language).

## ⚠️ IMPORTANT SECURITY AND PRIVACY NOTICE

**THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND.**

**THIS APPLICATION HAS NOT BEEN DESIGNED OR TESTED FOR PATIENT DATA CONFIDENTIALITY, SECURITY, OR REGULATORY COMPLIANCE.**

### Critical Limitations

- ❌ **NO ENCRYPTION**: Patient data is handled in plain text
- ❌ **NO ACCESS CONTROL**: No authentication or authorization mechanisms
- ❌ **NO AUDIT LOGGING**: No tracking of who printed what labels
- ❌ **NO DATA PROTECTION**: No secure storage or transmission of patient information
- ❌ **NOT HIPAA COMPLIANT**: Does not meet healthcare data protection standards
- ❌ **NOT PRODUCTION-READY**: Suitable for development/testing environments only

### Use at Your Own Risk

- You are solely responsible for compliance with healthcare regulations (HIPAA, GDPR, etc.)
- You are solely responsible for implementing appropriate security controls
- You are solely responsible for protecting patient confidentiality
- The authors assume NO LIABILITY for data breaches, compliance violations, or any damages

**If you handle real patient data, you MUST implement additional security measures including but not limited to: encryption, access controls, audit logging, secure networks, and compliance with applicable regulations.**

## Features

- **Automatic Printer Detection**: Detects all connected Zebra USB printers automatically via sysfs
- **Multiple Printer Support**: Automatically selects single printer or prompts user to choose when multiple are detected
- **Patient Information Labels**: Prints labels with:
  - Patient name (first and last)
  - Date of birth
  - Gender
  - Current date and time
  - Optional barcode
- **Date Validation**: Validates and formats dates (DD/MM/YYYY)
- **Batch Printing**: Print multiple copies of the same label
- **Error Recovery**: Automatic retry on print failures

## Requirements

### Hardware
- Zebra label printer (USB connected)
- Linux system with USB printer support

### Software
- Rust 1.70+ (Edition 2021)
- Cargo 1.93.0+

### Permissions
User must have write access to USB printer device (typically `/dev/usb/lp0`):
```bash
# Add user to lp group
sudo usermod -a -G lp $USER

# Or grant temporary access
sudo chmod 666 /dev/usb/lp0
```

## Installation

### From Source

1. Clone the repository:
```bash
git clone <repository-url>
cd label_printer
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run --release
```

### Binary Installation

After building, copy the binary to a location in your PATH:
```bash
sudo cp target/release/label_printer /usr/local/bin/
```

## Usage

Run the application:
```bash
cargo run
# or if installed
label_printer
```

The application will prompt for:
1. **First name**: Patient's first name
2. **Last name**: Patient's last name
3. **Date of birth**: Format as DDMMYYYY or DD/MM/YYYY
4. **Gender**: Patient's gender
5. **Barcode**: Whether to include a barcode (Y/N), and the barcode value if yes
6. **Number of labels**: How many copies to print

### Example Session

**Single printer:**
```
=== Label Printer ===
✓ Detected Zebra printer at: /dev/usb/lp0

--- New Label ---
First name: John
Last name: Smith
Date of birth (DDMMYYYY or DD/MM/YYYY): 15031985
Gender: M
Do you want to print a barcode? (Y/N): Y
Please enter a barcode: 12345678
Num Labels: 2

✓ Label sent to printer successfully!
✓ Label sent to printer successfully!
```

**Multiple printers:**
```
=== Label Printer ===
✓ Detected 3 Zebra printers:
  1. /dev/usb/lp0
  2. /dev/usb/lp1
  3. /dev/usb/lp2
Select printer (1-3): 2
✓ Selected: /dev/usb/lp1

--- New Label ---
First name: Jane
Last name: Doe
...
```

## Development

### Project Structure

```
label_printer/
├── src/
│   └── main.rs         # Main application logic
├── Cargo.toml          # Project manifest
└── README.md           # This file
```

## How It Works

1. **Printer Detection**: Scans `/sys/class/usbmisc` for all USB printer devices with Zebra vendor ID (0a5f)
2. **Printer Selection**: Automatically uses single printer or prompts user to select from multiple detected printers
3. **Data Collection**: Prompts user for patient information via interactive CLI
4. **Date Validation**: Parses and validates date of birth using chrono library
5. **ZPL Generation**: Generates Zebra Programming Language commands for label layout
6. **Printing**: Writes ZPL directly to the selected printer device file
7. **Error Handling**: Retries on failure with user confirmation

## ZPL Label Format

Labels are formatted with:
- 25pt font for text
- Positioned at fixed coordinates
- Uppercase formatting for names and gender
- Optional Code 128 barcode at 70pt height

## Dependencies

- **chrono** (0.4): Date and time handling

## Troubleshooting

### Printer Not Detected

Check USB connection:
```bash
lsusb | grep Zebra
```

Verify device exists:
```bash
ls -l /dev/usb/lp*
```

### Invalid Date Format

Date must be 8 digits (DDMMYYYY) or formatted with slashes (DD/MM/YYYY).
Example: `15031985` or `15/03/1985`

## License

MIT License

**THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.**

This software handles sensitive patient information. The authors and contributors:
- Make NO WARRANTIES about security, privacy, or regulatory compliance
- Accept NO RESPONSIBILITY for data breaches or confidentiality violations  
- Accept NO LIABILITY for any damages or consequences arising from use
- STRONGLY RECOMMEND implementing proper security controls before production use

See the [LICENSE](LICENSE) file for full terms.

## Contributing

When contributing, please:
1. Follow Rust standard formatting (`cargo fmt`)
2. Ensure code passes linting (`cargo clippy`)
3. Test manually with connected printer
