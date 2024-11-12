use homedir::my_home;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, process, vec};
use rand::Rng;
use homedir;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub image: String,
    pub format: Vec<String>
}

/*
${} to signify special format

${f[?]} - Format
 B = Bold
 I = Italic

${c[?]} - Color
 W = White
 B = Black
 R = Red
 B = Blue
 G = Green
 ? = Random

${r} - Repeat to Match previous line

${i[?]<#>} - Substitute Information
 L = Programming Language
 T = Rio Type
 P = Pathplanner Autos
 H = Version Hash
 M = Memory Usage
 N = Device name
 <#> - Max Pathplanner Autos

${b[?]} - Only continue if the requested bool is true (super niche)
 P = Path planner found
*/

pub struct RioProfile {
    pub programming_language: String,
    pub device_name: String,
    pub rio_type: String,
    pub path_planner: bool,
    pub path_planner_autos: Vec<String>,
    pub version_hash: String,
    pub memory_usage: i32,
    pub total_memory: i32,
}

pub fn apply_formats(cfg: &Config, profile: RioProfile) -> Vec<String> {
    let format_regex = Regex::new("\\$\\{.*?\\}").unwrap();
    let mut fnl: Vec<String> = vec!["".to_string()];

    for line in &cfg.format {
        let mut final_line: String = line.clone();

        let mut skip = false;
        for mat in format_regex.find_iter(&line) {
            let mat_str = &mat.as_str();
            let isolated_format = &mat_str[2..mat_str.len() - 1];

            let replacement: &str = match &isolated_format[..1] {
                // Format
                "f" => {  
                    let fmt = &isolated_format[1..2];
                    match fmt {

                        // Bold
                        "B" => {
                            "\x1b[1m"
                        },

                        // Italic
                        "I" => {
                            "\x1b[3m"
                        },

                        // Underline
                        "U" => {
                            "\x1b[4m"
                        },

                        // Blinking
                        "L" => {
                            "\x1b[5m"
                        },

                        // Strikethrough / Crossout
                        "S" => {
                            "\x1b[9m"
                        }
                        _ => {
                            eprintln!("Failed to parse configuration file {} is an invalid format identifier", fmt);
                            process::exit(1);
                        }
                    }
                },

                // Information
                "i" => {
                    let request = &isolated_format[1..2];

                    match request {

                        //Programming Language
                        "L" => {
                            &profile.programming_language
                        },

                        "T" => {
                            &profile.rio_type
                        },

                        "P" => {
                            &profile.path_planner_autos.join(" | ")

                            //TODO: Maybe extend over multiple lines like in demo
                        },

                        "H" => {
                            &profile.version_hash
                        },

                        "M" => {
                            &format!("{}%", profile.memory_usage / profile.total_memory)
                        },

                        "N" => {
                            &profile.device_name
                        }

                        _ => {
                            eprintln!("Failed to parse configuration file {} is an invalid information identifier", request);
                            process::exit(1);
                        }
                    }
                },

                // Match previous line / repeat
                "r" => {
                    if fnl.len() == 0 {
                        eprintln!("Failed to parse configuration file, repeat may not be used on the first line");
                        process::exit(1);
                    }
                    let repeated_character = &isolated_format[1..2];

                    &repeated_character.repeat(fnl[fnl.len() - 1].len())
                },

                // Color
                "c" => {
                    let color = &isolated_format[1..2];

                    match color {
                        "W" => {
                            "\x1b[97m"
                        },

                        "R" => {
                            "\x1b[91m"
                        },

                        "B" => {
                            "\x1b[94m"
                        },

                        "G" => {
                            "\x1b[92m"
                        },

                        "?" => {
                            let num = rand::thread_rng().gen_range(1..4);

                            match num {
                                1 => {
                                    "\x1b[91m" 
                                }

                                2 => {
                                    "\x1b[97m"
                                }

                                3 => {
                                    "\x1b[94m"  
                                }

                                4 => {
                                    "\x1b[92m"
                                }

                                _ => {
                                    "\x1b[91m" 
                                }
                            }
                        },

                        _ => {
                            eprintln!("Failed to parse configuration file, {} is not a valid color identifier", color);
                            process::exit(1);
                        }
                    }
                },
                
                // Bool check, kinda niche rn
                "b"  => {

                    let bool = &isolated_format[1..2];

                    match bool{
                        "P" => {
                            if !profile.path_planner {
                                skip = true;

                                break;
                            }

                            ""
                        },

                        "N" => {
                            if profile.path_planner {
                                skip = true;
                                break; 
                            }

                            ""
                        }

                        _ => {
                            eprintln!("Failed to parse configuraion file, {} is not a valid format", mat_str);
                            process::exit(1);
                        }
                    }
                },

                _ => {
                    eprintln!("Failed to parse configuration file, {} is not a valid format", mat_str);
                    process::exit(1);
                }
            };

            final_line = final_line.replace(mat_str, replacement);
        }

        let log_swap = match cfg.image.as_str() {
            "FIRST" => {
                "\x1b[53G".to_owned()
            },

            "NI" => {
                "\x1b[45G".to_owned()
            }

            _ => {
                "\x1b[45G".to_owned()
            }
        };

        if !skip { fnl.push(log_swap + &final_line + "\x1b[0m") }
    }

    fnl
}

pub fn fetch_config() -> Config {
    let home = my_home().expect("Couldn't find the home directory for the current user.");

    let config: Config = match home {
        Some(buffer) => {
            if !(buffer.join("riofetch.json").exists()) {
                let default = Config {
                    image: "NI".to_string(),
                    format: vec![
                        "${cB}${iN}${cW}@${cB}roborio.local".to_string(),
                        "${fB}${r-}".to_string(),
                        "${fB}Programming Language: ${fB}${c?}${iL}".to_string(),
                        "${fB}Rio Version: v${iT}".to_string(),
                        "${bP}${fB}Pathplanner Autos: ${iP}".to_string(),
                        "${bN}${fB}${cR}No Autos Located".to_string(),
                        "${fB}Memory Usage: ${iM}".to_string(),
                        "${fB}Version Hash: ${iH}".to_string()
                    ]
                };

                write_config(&default);

                return default;
            }

            let file_data = fs::read_to_string(buffer.join("riofetch.json")).unwrap();

            serde_json::from_str(&file_data).unwrap()
        }
        None => {
            eprintln!("Couldn't find the home directory for the current user.");
            process::exit(1);
        }
    };

    config
}

pub fn write_config(config: &Config) {
    let home = my_home().expect("Couldn't find the home directory for the current user.");


    match home {
        Some(buffer) => {
            fs::write(buffer.join("riofetch.json"), serde_json::to_string_pretty(&config).unwrap()).expect("Couldn't write config");
        }
        None => {
            eprintln!("Couldn't find the home directory for the current user.");
            process::exit(1);
        }
    }
}