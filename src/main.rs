use std::{env, process};

use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use components::{config::{self, RioProfile}, images};

mod components;

/*
riofetch
Programming Language: ... (Check for jar file / Executable / Python File) ✅
Identified Packages: ... (Check deploy for coreos / pathplanners) ✅
 - Auto 2024 Outside Stage
 - Auto Under Stage 2024 FRC
 - BackAuto
 - Mat Test
 (5 more autos)
Rio Type: 1 vs 2
Version Hash: ... (Hash published files)

Rio 1 - 256000 kb
Rio 2 - 512000 kb

!! DELETE EXAMPLE.TXT !!


Formatting Options:
${} to signify special format

${f[?]} - Format
 B = Bold
 I = Italic
 R = Reset

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
 <#> - Max Pathplanner Autos

*/

fn align_text_to_art(art: String, data: Vec<String>) -> String {
    let art_lined: Vec<&str> = art.split("\n").collect();
    let mut output_text: String = String::from("");

    for iterator in 1..(art_lined.len().max(data.len())) {

        if iterator < art_lined.len() {
            output_text += &art_lined[iterator];
        }

        if iterator < data.len() {
            output_text += &data[iterator];
        }

        output_text += "\n";
    }

    output_text
}

#[tokio::main]
async fn main() -> Result<(), async_ssh2_tokio::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "--demo" {
            let cfg = config::fetch_config();

            let fmts = config::apply_formats(&cfg, RioProfile {
                programming_language: "Java".to_string(),
                device_name: "2914".to_string(),
                rio_type: "2".to_string(),
                path_planner: true,
                path_planner_autos: vec!["Under Stage Auto".to_string(), "Far Right Auto".to_string(), "Far Left Auto".to_string(), "Shoot".to_string()],
                version_hash: "78207174ee60eab71a46e4451624f97f".to_string(),
                memory_usage: 856,
                total_memory: 1000
            });

            match cfg.image.as_str() {
                "FIRST" => {
                    println!("\n\n{}", align_text_to_art(images::FRCLOGO.to_string(),  fmts));
                },
                "NI" => {
                    println!("\n\n{}", align_text_to_art(images::NILOGO.to_string(),  fmts));
                },

                _ => {
                    eprintln!("Couldn't parse Config, \"FRC\" and \"NI\" are the only available images right now.");
                    process::exit(1);
                }
            }

            return Ok(())
        }
    }


    let auth_method = AuthMethod::with_password("");

    let _client = Client::connect("172.22.11.2", "lvuser", auth_method, ServerCheckMethod::NoCheck).await;

    match _client {
        Ok(client) => {
            let lang_output = client.execute("cat ./robotCommand").await.unwrap();
    
            let mut recognized_language = "Unidentified";
        
            match lang_output.stdout.to_lowercase() {
                out if out.contains(".jar") || out.contains("java") => recognized_language = "Java",
                out if out.contains("frcUserProgram") => recognized_language = "C++",
                out if out.contains("python") => recognized_language = "Python",
                _ => {}
            }
        
            let deploy_output = client.execute("ls deploy").await?;
        
            let pathplanner_found = deploy_output.stdout.contains("pathplanner");
            let mut pathplanner_autos: Vec<String> = vec![];
        
            if pathplanner_found {
                let path_out = client.execute("ls deploy/pathplanner/autos").await?;
                pathplanner_autos = path_out.stdout.split("  ").map(|s| s.to_string()).collect();
            }
        
            let choreo_found = deploy_output.stdout.contains("trajectories");
            let mut _choreo_autos: Vec<&str>; //TODO: Split .json
        
            if choreo_found {
                let choreo_output = client.execute("ls deploy/trajectories").await.unwrap();
        
                if choreo_output.stdout != "" {
                    _choreo_autos = choreo_output.stdout.split("  ").collect();
                }
            }
        
            let mem_grep = client.execute("grep MemTotal /proc/meminfo").await?;
        
            let memory_split: Vec<&str> = mem_grep.stdout.split("        ").collect();
            
            let isolated_number: Vec<&str> = memory_split[0].split(" ").collect();
            let total_memory = String::from(isolated_number[0]).trim().parse::<i32>().unwrap();

            let free_grep = client.execute("grep MemFree /proc/meminfo").await?;
            let free_split: Vec<&str> = free_grep.stdout.split("        ").collect();
            let free_isolated: Vec<&str> = free_split[0].split(" ").collect();

            let free_memory = String::from(free_isolated[0]).trim().parse::<i32>().unwrap();

            let memory_usage = total_memory - free_memory;

            let mut rio_version = "1";
        
            if total_memory > 300000 {
                rio_version = "2";
            }

            let recursive_out = client.execute("find . -type f -exec md5sum {} + | md5sum").await?;

            let hash_collection:Vec<_> = recursive_out.stdout.split(" ").collect();
            let hash = hash_collection[0];

            let hostname_cmd = client.execute("hostname").await?;

            let cfg = config::fetch_config();
            let profile = RioProfile {
                programming_language: recognized_language.to_string(),
                device_name: hostname_cmd.stdout,
                rio_type: rio_version.to_string(),
                path_planner: pathplanner_found,
                path_planner_autos: pathplanner_autos,
                version_hash: hash.to_string(),
                memory_usage: memory_usage,
                total_memory: total_memory
            };

            let fmts = config::apply_formats(&cfg, profile);


            match cfg.image.as_str() {
                "FIRST" => {
                    println!("\n\n{}", align_text_to_art(images::FRCLOGO.to_string(),  fmts));
                },
                "NI" => {
                    println!("\n\n{}", align_text_to_art(images::NILOGO.to_string(),  fmts));
                },

                _ => {
                    eprintln!("Couldn't parse Config, \"FRC\" and \"NI\" are the only available images right now.");
                    process::exit(1);
                }
            }
            Ok(())
        },
        Err(_) => {
            eprintln!("Wasn't able to locate a roborio, are you conected to the rio? \x1b[91m(If you want to test your config without a rio use --demo)");
            process::exit(1);
        }
    }
}