use std::iter;

use async_ssh2_tokio::client::{self, AuthMethod, Client, ServerCheckMethod};


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
Cpu usage:
Version Hash: ... (Hash published files)

Rio 1 - 256000 kb
Rio 2 - 512000 kb

⠀⠀⠀⠀⠀⠀⠀⡰⡐⡦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢴⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠐⡌⡲⡽⣝⡦⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢔⡽⣕⢷⢕⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢨⠢⡹⣮⡳⡽⣝⣖⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡰⡵⣻⣪⠯⣫⢯⡺⡮⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢅⢣⡻⣪⣞⡝⡮⣞⢽⡲⣄⠀⠀⠀⡀⢄⢠⠠⢄⢄⢄⢀⢤⢞⣞⡽⡕⡇⡣⠪⡱⡫⡯⣞⣞⡤⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢈⠪⡢⡯⣳⢵⠑⠸⡘⢷⢝⣞⢶⢌⡪⠘⠌⠂⠉⠊⠂⣱⢽⢝⣵⡳⢝⠌⠂⠀⠀⠈⠪⡙⢮⢞⡮⣗⢦⡀⠀⠀
⠀⠀⠀⠀⠀⡰⡑⡭⡯⣺⡕⠀⠀⠈⠪⡱⡳⣫⢗⣗⣄⠀⠀⠀⣠⢞⣵⡫⡯⢎⠪⠢⠢⠀⠀⠀⠀⠀⠁⢣⠹⡺⡵⣫⢞⡦⡀⠀⠀⠀
⠀⠀⠀⠀⠀⡢⡱⣝⢽⡺⡂⠀⠀⠀⠀⡊⠸⡘⢗⢧⢗⢷⢄⢮⢗⡯⣞⣎⢇⠕⠁⠀⠉⡂⠀⠀⠀⠀⠀⠀⠁⠕⣝⡵⣻⣪⢯⠂⠀⠀   
⠀⠀⠀⠀⢈⠆⢮⣺⢵⣫⠀⢀⠀⠀⠀⠀⠀⡈⡪⡩⣫⢯⣣⠱⡹⡺⣕⣗⢗⣄⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⣠⢯⡳⡽⣕⢇⠣⠁⠀⠀ 
⠀⠀⠀⠀⢢⢩⢺⣪⢗⡇⠀⢐⠀⠀⠀⠀⠱⢜⡮⣞⡵⣫⢞⡵⠥⢩⢚⢮⣻⣪⢷⠄⠀⠀⠀⠀⠔⠀⣠⢞⣗⡽⡺⢝⠔⠁⠀⠀⠀⠀
⠀⠀⠀⢀⠕⡌⣗⢷⢝⢆⢢⣑⢕⢄⠀⠀⠀⠀⠉⠚⠞⠉⠁⠀⠀⠀⠁⠇⠓⠑⠁⠀⠀⠀⡀⢎⢡⡺⡮⡯⣺⢎⢍⠆⠁⠀⠀⠀⠀⠀ 
⠀⠀⠀⢠⠣⡱⡯⣳⡫⣞⡵⣳⡳⡜⢌⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⢄⢪⢸⢼⢝⡽⣺⢝⠢⠑⠀⠀⠀⠀⠀⠀⠀⠀  
⠀⠀⠀⢅⠣⣝⣞⢵⣫⢗⡽⠚⠊⠉⠀⠑⠱⡑⡅⢆⢄⠄⡄⡀⢄⢠⠠⡠⡰⡐⢕⠸⡨⣺⡪⣗⢯⢏⠪⠂⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢐⡑⢕⣳⠵⠋⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⠂⠑⠱⠘⠌⠎⠢⠣⠑⠈⠈⠀⠀⠑⠰⡙⡕⡇⠕⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠈⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠕⠈⠀⠀⠀⠀⠀⠀

!! DELETE EXAMPLE.TXT !!
*/

const FRCLOGO: &str = "
\x1b[91m        ⡰⡐⡦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[94m⠀⠀⠀⠀⢀⢴⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⠀⠐⡌⡲⡽⣝⡦⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[94m⠀⠀⢀⢔⡽⣕⢷⢕⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⠀⢨⠢⡹⣮⡳⡽⣝⣖⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[94m⠀⢀⡰⡵⣻⣪⠯⣫⢯⡺⡮⣆⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⠀⢅⢣⡻⣪⣞⡝⡮⣞⢽⡲⣄⠀\x1b[97m⠀⠀⡀⢄⢠⠠⢄⢄⢄\x1b[94m⢀⢤⢞⣞⡽⡕⡇⡣⠪⡱⡫⡯⣞⣞⡤⡀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⢈⠪⡢⡯⣳⢵⠑⠸⡘⢷⢝⣞⢶\x1b[97m⢌⡪⠘⠌⠂⠉⠊⠂\x1b[94m⣱⢽⢝⣵⡳⢝⠌⠂⠀⠀⠈⠪⡙⢮⢞⡮⣗⢦⡀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⡰⡑⡭⡯⣺⡕⠀⠀⠈⠪⡱⡳⣫⢗⣗⣄⠀⠀\x1b[94m⠀⣠⢞⣵⡫⡯⢎⠪\x1b[97m⠢⠢⠀\x1b[94m⠀⠀⠀⠀⠁⢣⠹⡺⡵⣫⢞⡦⡀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠀⠀⡢⡱⣝⢽⡺⡂⠀⠀⠀⠀\x1b[97m⡊\x1b[91m⠸⡘⢗⢧⢗⢷⢄\x1b[94m⢮⢗⡯⣞⣎⢇⠕⠁⠀\x1b[97m⠉⡂⠀⠀⠀⠀⠀⠀\x1b[94m⠁⠕⣝⡵⣻⣪⢯⠂⠀⠀   \x1b[0m
\x1b[91m⠀⠀⠀⠀⢈⠆⢮⣺⢵⣫⠀\x1b[97m⢀⠀⠀⠀⠀⠀\x1b[91m⡈⡪⡩⣫⢯⣣\x1b[94m⠱⡹⡺⣕⣗⢗⣄⠀⠀⠀⠀⠀⠀\x1b[97m⠠⠀⠀⠀\x1b[94m⣠⢯⡳⡽⣕⢇⠣⠁⠀⠀ \x1b[0m
\x1b[91m⠀⠀⠀⠀⢢⢩⢺⣪⢗⡇⠀\x1b[97m⢐⠀⠀⠀⠀\x1b[91m⠱⢜⡮⣞⡵⣫⢞⡵\x1b[94m⠥⢩⢚⢮⣻⣪⢷⠄⠀⠀⠀⠀\x1b[97m⠔\x1b[94m⠀⣠⢞⣗⡽⡺⢝⠔⠁⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⢀⠕⡌⣗⢷⢝⢆⢢\x1b[97m⣑⢕⢄⠀⠀⠀⠀\x1b[91m⠉⠚⠞⠉⠁⠀⠀\x1b[94m⠀⠁⠇⠓⠑⠁⠀⠀⠀\x1b[97m⡀⢎⢡\x1b[94m⡺⡮⡯⣺⢎⢍⠆⠁⠀⠀⠀⠀⠀ \x1b[0m
\x1b[91m⠀⠀⠀⢠⠣⡱⡯⣳⡫⣞⡵⣳⡳\x1b[97m⡜⢌⠢⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⢄⢪⢸\x1b[94m⢼⢝⡽⣺⢝⠢⠑⠀⠀⠀⠀⠀⠀⠀⠀  \x1b[0m
\x1b[91m⠀⠀⠀⢅⠣⣝⣞⢵⣫⢗⡽⠚⠊\x1b[97m⠉⠀⠑⠱⡑⡅⢆⢄⠄⡄⡀⢄⢠⠠⡠⡰⡐⢕⠸⡨\x1b[94m⣺⡪⣗⢯⢏⠪⠂⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⢐⡑⢕⣳⠵⠋⠊⠁⠀⠀⠀⠀\x1b[97m⠀⠀⠀⠀⠈⠂⠑⠱⠘⠌⠎⠢⠣⠑⠈⠈\x1b[94m⠀⠀⠑⠰⡙⡕⡇⠕⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[91m⠀⠀⠀⠈⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\x1b[94m⠀⠁⠕⠈\x1b[0m";

const NILOGO: &str = "
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀   ⠨⡪⡣⡫⡪⡣⡫⡪⡣⡫⣒⢆⢄⠀⠀⠀⠀⠀⠀⡱⡹⡸⡱⡹⡸⡱⡅\x1b[0m⠀⠀⠀⠀⠀⠀⠀⠀
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⡪⡪⡎⡮⡪⡎⡮⡪⣪⢪⢪⢪⢣⠄⠀⠀⠀⠀⢜⢜⢜⢎⢎⢮⢪⡂⠀⠀⠀⠀\x1b[0m⠀⠀⠀⠀
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⡪⣪⢪⢪⢪⢪⢪⢪⢪⢪⡪⡎⡮⡪⡀⠀⠀⠀⢪⢪⢣⢣⢣⡣⡣⡃⠀⠀⠀⠀⠀\x1b[0m⠀⠀⠀
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀ ⢀⣀⣀⣀⣀⣀⣀⠨⠪⠪⠪⠣⠣⠓⠕⡕⡇⡇⡇⡇⡇⡇ ⠀⠀⡱⡱⡱⡕⡕⡕⣕⠅⠀⠀⠀⠀⠀\x1b[0m⠀⠀⠀
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢰⢒⢖⢲⢲⢸⠡⠀⠀⠀⠀⠀⠀⠀⡇⡇⡗⡕⣕⢕⢝⠄⠀⠀⠀⢜⢜⢎⢎⢮⢪⢪⡂⠀⠀⠀⠀⠀⠀\x1b[0m⠀⠀
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⢪⢪⡪⡪⡃⠀⠀⠀⠀⠀⠀⠀⡇⡇⣇⢇⢇⢇⢗⠅⠀⠀⠀⢪⢪⡪⡪⡪⣪⢪⡂⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⢜⢜⢜⢜⡌⠀⠀⠀⠀⠀⠀⠀⡇⡇⡇⡇⡗⡕⣕⠅⠀⠀⠀⡱⡱⡱⡹⡸⡸⡸⡄⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⢸⢸⢪⢕⠎⠀⠀⠀⠀⠀⠀⠀⡇⡇⡗⡕⣕⢕⢕⠅⠀⠀⠀⢸⢜⢎⢎⢮⢪⡪⡂⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⢱⢱⢱⢱⠅⠀⠀⠀⠀⠀⠀⠀⡇⡇⡧⡣⡣⡣⡫⡅⠀⠀⠀⠘⡜⡜⡜⡜⡜⣜⡂⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⢪⢪⢣⢣⡃⠀⠀⠀⠀⠀⠀⠀⡇⡇⡇⡇⡗⡕⣕⠅⠀⠀⠀⠀⠑⢕⢝⢜⢜⢆⠇⠀⠀⠀⠀⠀⠀⠀⠀\x1b[0m
\x1b[92m⠀⠀⠀⠀⠀⠀⠀⠀⢸⢸⢸⡸⡜⡜⡜⡆⠀⠀⠀⠀⠀⠀⠀⡇⡇⡏⡎⣎⢎⢎⠆⠀⠀⠀⠀⠀⠀⠑⠕⢕⡕⡅⠀\x1b[0m
";

fn align_text_to_art(art: String, data: Vec<&str>) -> String {
    let art_lined: Vec<&str> = art.split("\n").collect();
    let mut output_text: String = String::from("");
    for iterator in 1..(art_lined.len().max(data.len())) {

        if iterator < art_lined.len() {
            output_text += art_lined[iterator];
        }

        if iterator < data.len() {
            output_text += data[iterator];
        }

        output_text += "\n";
    }

    output_text
}

// #[tokio::main]
/*  async*/ fn main() -> Result<(), async_ssh2_tokio::Error> {


    println!("{}", align_text_to_art(FRCLOGO.to_string(), vec!["", "", "2914@roborio.local", "----------", "================"]));
    println!("{}", align_text_to_art(NILOGO.to_string(), vec!["", "", "2914@roborio.local", "----------", "================"]));
    Ok(())
    // let auth_method = AuthMethod::with_password("");

    // let client = Client::connect("172.22.11.2", "lvuser", auth_method, ServerCheckMethod::NoCheck).await.unwrap();

    // let lang_output = client.execute("cat ./robotCommand").await.unwrap();
    
    // let mut recognized_language = "Unidentified";

    // match lang_output.stdout.to_lowercase() {
    //     out if out.contains(".jar") || out.contains("java") => recognized_language = "Java",
    //     out if out.contains("frcUserProgram") => recognized_language = "C++",
    //     out if out.contains("python") => recognized_language = "Python",
    //     _ => {}
    // }

    // //TODO: Print logo of lang && first line

    // let deploy_output = client.execute("ls deploy").await.unwrap();

    // let pathplanner_found = deploy_output.stdout.contains("pathplanner");
    // let mut pathplanner_autos: Vec<&str>; //TODO: Split .auto

    // if pathplanner_found {
    //     let pathplanner_output = client.execute("ls deploy/pathplanner/autos").await.unwrap();

    //     if pathplanner_output.stdout != "" {
    //         pathplanner_autos = pathplanner_output.stdout.split("  ").collect();
    //     }
    // }

    // let choreo_found = deploy_output.stdout.contains("trajectories");
    // let mut choreo_autos: Vec<&str>; //TODO: Split .json

    // if choreo_found {
    //     let choreo_output = client.execute("ls deploy/trajectories").await.unwrap();

    //     if choreo_output.stdout != "" {
    //         choreo_autos = choreo_output.stdout.split("  ").collect();
    //     }
    // }

    // let mem_grep = client.execute("grep MemTotal /proc/meminfo").await?;

    // let memory_split: Vec<&str> = mem_grep.stdout.split("        ").collect();
    
    // let isolated_number: Vec<&str> = memory_split[0].split(" ").collect();
    // let total_memory = String::from(isolated_number[0]).trim().parse::<i32>().unwrap();

    // let mut rio_version = "v1";

    // if total_memory > 300000 {
    //     rio_version = "v2";
    // }
    

    // Ok(())
}