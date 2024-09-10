/*
 * Dru Banks (s0krat3z)
 * Blue Cord Security
 * Version 1.0.0
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashSet; // Useful for storing unique subdomains
use std::error::Error;
use clap::{Command, Arg};
use reqwest;
use serde::Deserialize;
use colored::*;

const BANNER: &str = r#"
_________                __   _________             
\_   ___ \  ____________/  |_/   _____/_____ ___.__.
/    \  \/_/ __ \_  __ \   __\_____  \\____ <   |  |
\     \___\  ___/|  | \/|  | /        \  |_> >___  |
 \______  /\___  >__|   |__|/_______  /   __// ____|
        \/     \/                   \/|__|   \/    
            For Legal and Ethical Use Only

"#;

#[derive(Debug, Deserialize)] //Struct for certificate info after JSON deserialization
struct CertificateInfo {
    name_value: String,
}

async fn get_subdomains(domain: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    println!("{}", "Scanning for public subdomains...".green());
    let url: String = format!("https://crt.sh/?q=%.{}&output=json", domain);
    let response = reqwest::get(&url).await?;
    let data: Vec<CertificateInfo> = response.json().await?;
    let mut subdomains = HashSet::new();
    for item in data {
        let name_value = item.name_value;
        if name_value.contains('\n') {
            let subname_value: Vec<&str> = name_value.split('\n').collect();
            for subname in subname_value {
                if !subname.contains('*') {
                    subdomains.insert(subname.to_string());
                }
            }
        }
    }
    Ok(subdomains)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", BANNER);
    let matches = Command::new("CertSPY-RS")
        .version("1.0.0")
        .author("Dru Banks (S0KRAT3Z)")
        .about("Rust subdomain enumeration tool using crt.sh")
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("Website to enumerate")
                .required(true)
        )
        .get_matches();

    let domain = matches.get_one::<String>("domain").unwrap();
    let subdomains = get_subdomains(domain).await?;
    if !subdomains.is_empty() {
        println!("{}", format!("[*] Subdomains of {}:", domain).green());
        for subdomain in subdomains {
            println!("{}", subdomain.yellow());
        }
    } else {
        println!("{}", format!("[-] No subdomains found for {}", domain).red());
    }
    Ok(())
}



