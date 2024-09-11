        
# CertSpy-rs
CertSpy-rs is a Rust-based subdomain enumeration tool that uses the crt.sh Certificate Transparency logs to discover subdomains for a given domain.
It is a Rust port of my CertSpy tool that was originally written in Python.


## Features
- Fast and efficient subdomain enumeration
- Uses crt.sh Certificate Transparency logs
- Colorized output for better readability
- Asynchronous I/O for faster results
- Filters out wildcard subdomains

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the repository: git clone https://github.com/SOKRAT3Z/certspy-rs.git && cd certspy-rs

3. Build the project: cargo build --release

4. The compiled binary will be available in `target/release/certspy-rs`

## Usage
`certspy-rs -d/--domain <domain>`
## Output

CertSpy-RS will display the discovered subdomains in the terminal. Subdomains will be printed in yellow for better visibility.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This tool is for educational and ethical use only. The author is not responsible for any misuse or damage caused by this program. Always ensure you have explicit permission to test or enumerate subdomains on a domain.

## Contact

For any queries or suggestions, please open an issue on this repository.
