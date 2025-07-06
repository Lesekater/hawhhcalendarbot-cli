// Deine ursprüngliche Formatierung und Kommentare werden jetzt berücksichtigt

use clap::builder::Str;
use reqwest::blocking::Client;
use std::{error::Error};
use scraper::{Html, Selector};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::fs;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
#[derive(Debug, Serialize, Deserialize)]
pub struct MupLecture {
    name: String,
    location: String,
    description: String,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl MupLecture {
    /*########################################
    Web Scraper:
    ########################################*/

    fn generate_urls(base_url: &str, custom_ending: &str, ending: &str, iter: i8) -> Vec<String> {
        (1..iter)
            .map(|i| format!("{}{}{}{}", base_url, custom_ending, i, ending))
            .collect()
    }

    pub fn scrape_lecture_plan(user: String, password: String, url: String, semester_groupe: String) -> Result<Vec<MupLecture>, Box<dyn Error>> {
        let body = Self::fetch_html(user, password, &url)?;
        let lecture_table = Self::extract_lecture_table(&body)?;
        let lecture_structs = Self::parse_lecture_table(lecture_table, semester_groupe);

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

    fn parse_lecture_table(lecture_table: Vec<Vec<(String, usize, Option<String>)>>, semester_group: String) -> Vec<MupLecture> {
        let mut lectures = Vec::new();

        for (i, rows) in lecture_table.iter().enumerate() {
            for (j, (content, rowspan, title)) in rows.iter().enumerate() {
                if !content.is_empty() && i > 0 && j > 0 {
                    if let Some(parsed_infos) = Self::parse_lecture_info(content) {
                        for (name, prof, location, description) in parsed_infos {
                            let (start_time, end_time) = Self::calc_lecture_hours(i, *rowspan);
                           
                           let mut disc = String::new();
                            match title {
                                Some(titl) => { 
                                    if titl.len() < 22 {
                                        disc = format!("full Name: {}, {}, Day: {}", titl, description, LectureDay::from_index(j));
                                    } else {
                                        disc = format!("{},\nDay: {}", description, LectureDay::from_index(j));
                                    }

                                },
                                None => {}
                            }

                            lectures.push(MupLecture {
                                name: format!("{}-{}", semester_group, name),
                                location,
                                description: disc,
                                start: start_time,
                                end: end_time,
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

        let re_location = Regex::new(r"BT(\d+)-(\d+)").ok()?;
        let (loc_vec, loc_pos_vec): (Vec<_>, Vec<_>) = re_location
            .find_iter(input)
            .map(|m| (m.as_str().to_string(), m.end()))
            .unzip();

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

            let le_prof = Regex::new(r"[A-Z][a-z]{2,3}").ok()?;
            let prof_scope = &classes[..classes.len().min(60)];
            let prof = le_prof.find(prof_scope)?.as_str().to_string();

            let patterns = [
    // Bestehende Patterns (deine Liste)
                r"\b[A-Z]{3} [LU]\b",
                r"\b[A-Z]{3}\.?\b",
                r"\b[A-Z]{3}\b",
                r"\b[A-Z]+-\d\b",
                r"\b[A-Z][a-z]+ \d(?: [A-Z])?\b",
                r"\b[A-Z]{2,4} [A-Z](?: [U])?\b",
                r"\b[A-Z][a-z]+ [PM]\b",
                r"\b[A-Z]{4,}\b",
                r"\b[A-Z]{4,} L\b",
                r"\bAT\d(?: L)?\b",
                r"\b[A-Z][a-z]{3}\b",
                r"\b_[A-Z]{3,}_(?: [A-Z])?\b",
                r"\b[A-Z]{2}[a-z]{2}\b",
                r"\b[A-Z]{3,5}\b",
                r"\b[A-Z]{2}-[a-z]{3,6}\b",
                r"\b[A-Z]{2,5}-\d[a-z]{2,4}\b",
                r"\b[A-Z]{4,}\.?\b",
                r"\b[A-Z]-[A-Z]{2,}\b",
            ];


            let vec_name_regex: Vec<Regex> = patterns
                .iter()
                .map(|pat| Regex::new(pat).ok())
                .collect::<Option<Vec<_>>>()?;

            let search_scope = classes.split_whitespace().take(7).collect::<Vec<_>>().join(" ");

            let all_matches: Vec<&str> = vec_name_regex
                .iter()
                .filter_map(|re| re.find(&search_scope).map(|m| m.as_str()))
                .collect();

            let mut all_matches_without_teams: Vec<&str> = all_matches.iter().copied().filter(|s| *s != "TEAMS").collect();

            all_matches_without_teams.sort_by_key(|m| -(m.len() as isize));
            let name = all_matches_without_teams.first().cloned().unwrap_or("").to_string();

            let mut description = classes.to_string();
            description = description.replace(&name, "");
            description = description.replace(&prof, "");
            description = description.replace(&location, "");
            description = description.trim().to_string();

            let dis = format!("Professor: {}, Discription: {}", prof, description);

            output_vec.push((name, prof, location, dis));
        }

        Some(output_vec)
    }

    fn calc_lecture_hours(index: usize, rowspan: usize) -> (NaiveDateTime, NaiveDateTime) {
    let time_slots = [
        ("08:15", "09:45"),
        ("10:00", "11:30"),
        ("12:15", "13:45"),
        ("14:00", "15:30"),
        ("15:45", "17:15"),
        ("17:30", "19:00"),
    ];

    // Dummy-Datum verwenden, da nur Zeit relevant ist
    let dummy_date = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    if index == 0 || index > time_slots.len() {
        // Rückgabe von minimalen gültigen Zeitpunkten, alternativ kannst du Option<T> verwenden
        return (
            NaiveDateTime::new(dummy_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
            NaiveDateTime::new(dummy_date, NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        );
    }

    let (start_str, end1_str) = time_slots[index - 1];

    let end_str = match rowspan {
        1 => end1_str,
        2 => {
            if index < time_slots.len() {
                time_slots[index].1
            } else {
                end1_str // Falls rowspan zu lang ist: Zeit des aktuellen Blocks
            }
        }
        _ => end1_str, // Fallback auf einfachen Block
    };

    let start_time = NaiveTime::parse_from_str(start_str, "%H:%M").unwrap();
    let end_time = NaiveTime::parse_from_str(end_str, "%H:%M").unwrap();

    (
        NaiveDateTime::new(dummy_date, start_time),
        NaiveDateTime::new(dummy_date, end_time),
    )
}

    /*########################################
    Haw bot Interface
    ########################################*/

    fn save_struct_to_json(structs: &Vec<Vec<MupLecture>>) -> std::io::Result<()> {
        let base_path = dirs::cache_dir().unwrap().join("hawhhcalendarbot-cli/eventdata/maschienenbau-und-produktion/");
        fs::create_dir_all(&base_path)?;
        

        for semester in structs {
            for (index,lectures) in semester.into_iter().enumerate() {
                
            let path = base_path.join(format!("{}.json",lectures.name.chars().map(|c| if c == ' ' { '-' } else { c }).collect::<String>()/*SemesterGroup::from_index(index)*/));
            

            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, lectures)?;
            }
        }
        
        Ok(())
    }

    pub fn load_struct_from_json() -> std::io::Result<Vec<Vec<MupLecture>>> {
        let path = dirs::cache_dir().unwrap().join("hawhhcalendarbot-cli/eventdata/mechatronik/");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }

    pub fn fetch_all_plans(user: String, password: String) -> Result<Vec<Vec<MupLecture>>, Box<dyn Error>> {
        let base_url = "https://www.mp.haw-hamburg.de/auth/vorlesungsplan/";
        let mut urls = Self::generate_urls(base_url, "B_MT", ".php", 7);

        let mut lectures: Vec<Vec<MupLecture>> = Vec::new();

        let all_urls: Vec<String> = vec![
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/1a.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/1en.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/2a.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/2b.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/2c.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/3a.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/4DM.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/4ET.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/4EK.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/4P.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/5_6DM.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/5_6ET.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/5_6EK.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/5_6P.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/Master_BS.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/Master_NE.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/Master_P.php"),
    String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/Master_KP.php"),
        ];
        urls.extend(all_urls);

        for (index, url) in urls.into_iter().enumerate() {
            let semester_name = Self::extract_last_segment(&url);
            lectures.push(Self::scrape_lecture_plan(user.clone(), password.clone(), url.clone(), semester_name)?);
        }

        Ok(lectures)
    }

    fn extract_last_segment(path: &str) -> String {
    path.rsplit('/')
        .next()
        .and_then(|filename| filename.strip_suffix(".php"))
        .unwrap_or("")
        .to_string()
}



    pub fn fetch_one_semester(user: String, password: String, semester_name: String) -> Result<Vec<MupLecture>, Box<dyn Error>> {
        let base_url = "https://www.mp.haw-hamburg.de/auth/vorlesungsplan/";
        let urls = Self::generate_urls(base_url, "B_MT", ".php", 7);

        let lectures = Self::scrape_lecture_plan(user.clone(), password.clone(), urls[0].clone(), semester_name)?;
        Ok(lectures)
    }

    pub fn fetch_all_mup_plans_to_cache(user: String, password: String) -> Result<(), Box<dyn Error>> {
        let plans = Self::fetch_all_plans(user, password)?;
        Self::save_struct_to_json(&plans)?;
        Ok(())
    }
}


#[derive(Debug)]
pub(crate) enum LectureDay {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday,
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

pub(crate) enum SemesterGroup {
    BMT1, BMT2, BMT3, BMT4, BMT5, BMT6,
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
