use reqwest::blocking::Client;
use std::{error::Error, ops::Index};
use scraper::{Html, Selector};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
//use serde_json::Result; 

#[derive(Debug)]
enum LectureDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl LectureDay {
    fn from_index(index: usize) -> String {
        match index {
            1 => String::from("Monday"),
            2 => String::from("Tuesday"),
            3 => String::from("Wednesday"),
            4 => String::from("Thursday"),
            5 => String::from("Friday"),
            6 => String::from("Saturday"),
            _ => String::from("Day not found!"),
        }
    }
}

enum SemesterGroup {
    BMT1,
    BMT2,
    BMT3,
    BMT4,
    BMT5,
    BMT6,
}

impl SemesterGroup {
    fn from_index(index: usize) -> String {
        match index {
            0 => String::from("BMT1"),
            1 => String::from("BMT2"),
            2 => String::from("BMT3"),
            3 => String::from("BMT4"),
            4 => String::from("BMT5"),
            5 => String::from("BMT6"),
            _ => String::from("Semester group not found!"),
        }

    }
    
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Lecture {
    name: String,
    location: String,
    description: Option<String>,
    start: Option<String>,
    end: Option<String>,
    hours: String,
}
/*
fn main() -> Result<(), Box<dyn Error>> {
    let base_url = "https://www.mp.haw-hamburg.de/auth/vorlesungsplan/";
    let urls = generate_urls(base_url, "B_MT", ".php", 7);

    //Top secret Info:
    let user = String::from("v2632340");
    let password = String::from("testPasswort1");

    let lecture_list = fetch_all_plans(user, password)?;

    println!("Vor dem Speichern: \n {:?}", lecture_list);


    //Speicher:
    let _ = save_struct_to_json(&lecture_list);

    let laoded_lecture_list = load_struct_from_json()?;


    let path = dirs::cache_dir().unwrap().join("hawhhcalendarbot\\Mechatronik");
    println!("Speicherpfad: {:?}", path);

    println!("########################################");

    println!("{:?}", laoded_lecture_list);

    println!("########################################");

    Ok(())
}*/

/*########################################
Web Scraper:
########################################*/

fn generate_urls(base_url: &str, custom_ending: &str, ending: &str, iter: i8) -> Vec<String> {
    (1..iter)
        .map(|i| format!("{}{}{}{}", base_url, custom_ending, i, ending))
        .collect()
}

pub fn scrape_lecture_plan(user: String, password: String, url: String, semester_index: usize) -> Result<Vec<Lecture>, Box<dyn Error>> {
    let body = fetch_html(user, password, &url)?;
    let lecture_table = extract_lecture_table(&body)?;
    let lecture_structs = parse_lecture_table(lecture_table, semester_index);

    Ok(lecture_structs)
}

fn fetch_html(user: String, password: String, url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()?;

    let response = client
        .get(url)
        .basic_auth(user, Some(password))
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Seitenabruf fehlgeschlagen: {}", response.status()).into());
    }

    Ok(response.text()?)
}

fn extract_lecture_table(body: &str) -> Result<Vec<Vec<(String, usize, Option<String>)>>, Box<dyn Error>> {
    let document = Html::parse_document(body);
    let table_selector = Selector::parse("table").unwrap();
    let mut rows_parsed = Vec::new();

    for table in document.select(&table_selector) {
        if let Some(style_attr) = table.value().attr("style") {
            if style_attr.contains("border-collapse") && style_attr.contains("background-color:#F7F8F8") {
                let row_selector = Selector::parse("tr").unwrap();
                let cell_selector = Selector::parse("td").unwrap();

                for row in table.select(&row_selector) {
                    let mut cells: Vec<(String, usize, Option<String>)> = Vec::new();

                    for cell in row.select(&cell_selector) {
                        let content = cell
                            .text()
                            .collect::<String>()
                            .replace('\u{a0}', " ")
                            .replace('\n', " ")
                            .replace('\r', " ")
                            .replace('»', " ")
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .join(" ");

                    let mut title = None;
                    let a_selector = Selector::parse("a").unwrap();
                    if let Some(a_tag) = cell.select(&a_selector).next() {
                        title = a_tag.value().attr("title").map(|s| s.to_string());
                    }
                            
                        let rowspan = cell
                            .value()
                            .attr("rowspan")
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(1);

                        cells.push((content, rowspan, title));
                    }

                    rows_parsed.push(cells);
                }

                break; // Nur erste passende Tabelle analysieren
            }
        }
    }

    Ok(rows_parsed)
}

fn parse_lecture_table(lecture_table: Vec<Vec<(String, usize, Option<String>)>>, semester_group_index: usize) -> Vec<Lecture> {
    let mut lectures = Vec::new();

    for (i, rows) in lecture_table.iter().enumerate() {
        for (j, (content, rowspan, title)) in rows.iter().enumerate() {
            if !content.is_empty() && i > 0 && j > 0 {
                if let Some(parsed_infos) = parse_lecture_info(content) {
                    //println!("{i}");
                    for (name, prof, location, description) in parsed_infos {
                        

                        let (start_time, end_time) = calc_lecture_hours(i, *rowspan);
                        
                        lectures.push(Lecture {
                            name: format!("{}-{}", SemesterGroup::from_index(semester_group_index), name),
                            location,
                            description: Some(description),
                            start: None,//Some(format!("Day: {}, Lecture start: {}", LectureDay::from_index(j), start_time)),
                            end: None,//Some(format!("Day: {}, Lecture end: {}", LectureDay::from_index(j), end_time)),
                            hours: format!("{}, from {} Uhr till {} Uhr", LectureDay::from_index(j), start_time, end_time),
                        });
                    }
                } else {
                    println!("Keine gültigen Vorlesungsinformationen gefunden in: '{}'", content);
                }
            }
        }
    }

    lectures
}

fn parse_lecture_info(input: &str) -> Option<Vec<(String, String, String, String)>> {
    if input.split_whitespace().count() < 3 {
        return None;
    }

    //Regex für die Raumnummer, ist am Sichersten, dass die gefunden wird.s
    let re_location = Regex::new(r"BT(\d+)-(\d+)").ok()?;
    let (loc_vec, loc_pos_vec): (Vec<_>, Vec<_>) = re_location
        .find_iter(input)
        .map(|m| (m.as_str().to_string(), m.end()))
        .unzip();

    //Regex für die "2 SW", also Start der Vorlesung.
    let re_semester_week = Regex::new(r"ab(\d+) SW").ok()?;
    let _ = re_semester_week.find_iter(input).collect::<Vec<_>>(); // Aktuell ungenutzt

    //Falls im Input mehrere Vorlesungen sind, werden diese hier Aufgeteilt:
    let mut input_split: Vec<String> = loc_pos_vec
        .iter()
        .copied()
        .chain(std::iter::once(input.len()))
        .scan(0, move |start, end| {
            let part = input[*start..end].to_string();
            *start = end;
            Some(part)
        })
        .collect();

    /*
    Durch das Auteilen kann es dazu kommen, dass zusatz Infos nicht in den Zugehörigen String gekommen sind.
    Hier wird überprüft ob in einem String eine Raum nummer ist, wenn nicht, wird dieser String an den String
    davor angehangen.
     */
    for i in 0..input_split.len() {
        let current = &input_split[i];
        if re_location.find(current).is_none() && i > 0 {
            input_split[i - 1] = format!("{} {}", input_split[i - 1], current);
            input_split.remove(i);
            break;
        }
    }

    let mut output_vec = Vec::new();

    for classes in input_split {
        let location = re_location.find(&classes)?.as_str().to_string();

        // Prof-Regex
        let le_prof = Regex::new(r"[A-Z][a-z]{2,3}").ok()?;
        let prof_scope = &classes[..classes.len().min(60)];
        let prof = le_prof.find(prof_scope)?.as_str().to_string();

        // Namenspatterns
        let patterns = [
            r"\b[A-Z]{3} [LU]\b",                    // RTT L, TM A U
            r"\b[A-Z]{3}\.?\b",                      // RTT, WZM
            r"\b[A-Z]{3}\b",                         // MDY
            r"\b[A-Z]+-\d\b",                        // MAT-1
            r"\b[A-Z][a-z]+ \d(?: [A-Z])?\b",        // Kon 3, Kon 3 L (Groß + Klein)
            r"\b[A-Z]{2,4} [A-Z](?: [U])?\b",        // TM A, TM B U
            r"\b[A-Z][a-z]+ [PM]\b",                 // Mkon P
            r"\b[A-Z]{4,}\b",                        // ILOG, FLUIDT
            r"\b[A-Z]{4,} L\b",                      // FLUIDT L
        ];



        let vec_name_regex: Vec<Regex> = patterns
            .iter()
            .map(|pat| Regex::new(pat).ok())
            .collect::<Option<Vec<_>>>()?;

        // Eingrenzung auf ersten Teil der Zeichenkette
        let search_scope = classes.split_whitespace().take(7).collect::<Vec<_>>().join(" ");

        //println!("eingegrenzte Suche: {}", search_scope);

        let all_matches: Vec<&str> = vec_name_regex
            .iter()
            .filter_map(|re| {
                re.find(&search_scope).map(|m| m.as_str())
            })
            .collect();

        let mut all_matches_without_teams: Vec<&str> = all_matches.iter().copied().filter(|s| *s != "TEAMS").collect();

        // Längsten Match wählen
        all_matches_without_teams.sort_by_key(|m| -(m.len() as isize));
        let name = all_matches_without_teams.first().cloned().unwrap_or("").to_string();

        //println!("Vorlesungsname: {}", name);

        //let description: String = String::new(); // noch leer
        let mut description = classes.to_string();
        description = description.replace(&name, "");
        description = description.replace(&prof, "");
        description = description.replace(&location, "");
        description = description.trim().to_string();
        
        let dis = format!("Professor: {}, weiter Informationen: {}", prof, description);

        //let re_extra_info = Regex::new(r"\d)").ok()?;
        //let extra_info = re_extra_info.find(&description);
        //println!("Extra Info: {:?}", extra_info);

        output_vec.push((name, prof, location, dis));
        //println!("Infos vor dem Matchen: {:?}", output_vec);
    }
    Some(output_vec)
}

fn calc_lecture_hours(index: usize, rowspan: usize) -> (String, String) {
/*die Uhrzeit soll nach den Zeilen des "i" for-loops sowie dem rowspan, beides usize, berechnet werden, also:
i = 0 -> kann ignoriert werden.
i = 1, rowspan = 1 -> start: 8.15 end: 9.45
i = 1, rowspan = 2 -> start: 8.15 end: 11.30

i = 2, rowspan = 1 -> start: 10.00 end: 11.30
i = 2, rowspan = 2 -> start: 10.00 end: 13:45

i = 3, rowspan = 1 -> start: 12.15 end: 13:45
i = 3, rowspan = 2 -> start: 12.15 end: 15:30

i = 4, rowspan = 1 -> start: 14:00 end: 15:30
i = 4, rowspan = 2 -> start: 14.00 end: 17:15

i = 5, rowspan = 1 -> start: 15.45 end: 17:15
i = 5, rowspan = 2 -> start: 15.45 end: 19:00

i = 6, rowspan = 1 -> start: 17:30 end: 19:00
i = 6, rowspan = 2 -> start: 17:30 end: -----

*/
    let time_slots = [
        ("08:15", "09:45"),
        ("10:00", "11:30"),
        ("12:15", "13:45"),
        ("14:00", "15:30"),
        ("15:45", "17:15"),
        ("17:30", "19:00"),
    ];

    if index == 0 || index > time_slots.len() {
        return ("".to_string(), "".to_string()); // i=0 oder ungültig
    }

    let (start, end1) = time_slots[index - 1];
    let end = match rowspan {
        1 => end1,
        2 => {
            // Bei rowspan = 2 muss nächster Zeitslot existieren
            if index < time_slots.len() {
                time_slots[index].1
            } else {
                "-----"
            }
        },
        _ => "-----", // nur rowspan 1 oder 2 erlaubt
    };

    (start.to_string(), end.to_string())
}

/*########################################
Haw bot Interface
########################################*/

fn save_struct_to_json(structs:  &Vec<Vec<Lecture>>) -> std::io::Result<()> {

    //Wir im Cache gespeichert:
    let path = dirs::cache_dir().unwrap().join("hawhhcalendarbot\\Mechatronik");

    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, structs)?;
    Ok(())

}

fn load_struct_from_json() -> std::io::Result<Vec<Vec<Lecture>>> {
    //Wir im Cache gespeichert:
    let path = dirs::cache_dir().unwrap().join("hawhhcalendarbot\\Mechatronik");

    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)

}


pub fn fetch_all_plans(user: String, password: String) -> Result<Vec<Vec<Lecture>>, Box<dyn Error>>{
    let base_url = "https://www.mp.haw-hamburg.de/auth/vorlesungsplan/";
    let urls = generate_urls(base_url, "B_MT", ".php", 7);

    let mut lectures: Vec<Vec<Lecture>> = Vec::new();

    for (index, url) in urls.into_iter().enumerate() {

        lectures.push(scrape_lecture_plan(user.clone(), password.clone(), url.clone() , index)?);
        //println!("{:#?}", lectures);
    }

    

    Ok(lectures)
}

pub fn fetch_one_semester(user: String, password: String, semester_index: usize) -> Result<Vec<Lecture>, Box<dyn Error>>{

    let base_url = "https://www.mp.haw-hamburg.de/auth/vorlesungsplan/";
    let urls = generate_urls(base_url, "B_MT", ".php", 7);

    let lectures = scrape_lecture_plan(user.clone(), password.clone(), urls[semester_index].clone() , semester_index)?;
    //println!("{:#?}", lectures);
    
    Ok(lectures)
}