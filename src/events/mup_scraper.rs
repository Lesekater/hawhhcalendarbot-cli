use reqwest::blocking::Client;
use std::error::Error;
use scraper::{Html, Selector};
use regex::Regex;

/*
zum einfacherren Speichern der Vorlesungstagen eine enum:
*/
#[derive(Debug)]
enum lecutre_days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl lecutre_days {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(lecutre_days::Monday),
            1 => Some(lecutre_days::Tuesday),
            2 => Some(lecutre_days::Wednesday),
            3 => Some(lecutre_days::Thursday),
            4 => Some(lecutre_days::Friday),
            5 => Some(lecutre_days::Saturday),
            _ => None,
        }
    }
}


/*
Zum einfacheren Speichern der Uhrzeiten ein enum:
*/

//Format zum speichern der Vorlesungsinformation:
/*
{
		"name": "B-Seminar_B",
		"location": "1283",
		"description": "Dozent: SML/STG",
		"start": "2025-04-15T16:00:00",
		"end": "2025-04-15T18:00:00"
	},
*/
#[derive(Debug)]
struct Lecture {
    name: String,
    location: String,
    description: Option<String>,
    start: Option<String>, 
    end: Option<lecutre_days>,
    hours: Option<String>,
}



//#Struktur zum speichern der Logindaten aus json oder ähnlichen:
//todo!("loader für die Userdaten implementieren");
//todo!("besseren weg zum login finden. Evtl. verschlüsseln der Daten oder Lokal speichern");
struct login_data {
    user: String,
    password: String,
}

/*
fn main()  -> Result<(), Box<dyn Error>> {
    let login = login_data {
        user: String::from("v2632340"),
        password: String::from("testPasswort1"),
    };

    let base_url = String::from("https://www.mp.haw-hamburg.de/auth/vorlesungsplan/");
    let urls = generat_url(&base_url, String::from("B_MT"), String::from(".php"), 7);

/*
    for url in urls {
        let lecutres = scrape_lecute_plan(&login, url.clone())?;
        println!("{}", url);
        println!("{:#?}", lecutres);
    }
     */
    let lecutres = scrape_lecute_plan(&login, urls[5].clone())?;
    println!("{:#?}", lecutres);

    

    Ok(())
}
 */

/* Beispiel für Aufruf
Base String: https://www.mp.haw-hamburg.de/auth/vorlesungsplan/
custom_ending: B_MT    -> das custom_ende wird an den Base link angefügt und das "x" wird iteriert und eingesetzt
                           also z.B. B_MT1, B_MT2, ..., B_MT7
ending: gibt die Dateinendung an z.B. ".php", kommt an das ende des Strings
iter: gibt die Anzahl an iteration über das "x" an, also bei iter = 5 -> B_MT1, ..., B_MT5
*/
pub fn generat_url(base_url: &String, custom_ending: String, ending: String, iter: i8) -> Vec<String> {
    let mut url_vec: Vec<String> = Vec::new();

    for i in 1..iter {
        url_vec.push(format!("{}{}{}{}", base_url, custom_ending, i, ending));
    } 

    url_vec
}

/*
kann nach dem login_mup() aufgerufen werden und gibt
*/
pub fn scrape_lecute_plan(login: &login_data, url: String) -> Result<Vec<Lecture>, Box<dyn Error>> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()?;

    let response = client
        .get(&url)
        .basic_auth(&login.user, Some(&login.password))
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Seitenabruf fehlgeschlagen: {}", response.status()).into());
    }

    println!("Inhalt erhalten");

    let body = response.text()?;
    //println!("Body-Ausschnitt:\n{}", &body[..1000]);

    let document = Html::parse_document(&body);
    let table_selector = Selector::parse("table").unwrap();

    let mut lecture_plan_raw: Vec<Vec<(String, usize)>> = Vec::new();

    //Stundenplan extrahiren in Vec<Vec<(String, usize))>> form!
    for table in document.select(&table_selector) {
    if let Some(style_attr) = table.value().attr("style") {
        if style_attr.contains("border-collapse") && style_attr.contains("background-color:#F7F8F8") {
            let row_selector = Selector::parse("tr").unwrap();
            let cell_selector = Selector::parse("td").unwrap();

           

            for row in table.select(&row_selector) {
                let mut cells: Vec<(String, usize)> = Vec::new(); // (Text, Rowspan)

                for cell in row.select(&cell_selector) {
                // Text bereinigen
                    let content = cell
                        .text()
                        .collect::<String>()
                        .replace('\u{a0}', " ")
                        .replace('\n', " ")
                        .replace('\r', " ")
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ");

                 // rowspan auslesen (Standardwert = 1)
                    let rowspan = cell
                        .value()
                        .attr("rowspan")
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(1);

                    cells.push((content, rowspan));
                }

                lecture_plan_raw.push(cells);
                

            }
            //um nur die Tabelle mit den Vorläsungen zu analysieren.
            break; 
            //println!("{:?}", lecture_plan_raw)
        }
    }
    
    }

    let mut lecture_in_struct: Vec<Lecture> = Vec::new();

    //Bereinigen (Uhrzeiten entfernen)
    for (i, rows) in lecture_plan_raw.iter().enumerate() {
        for (j, (content, rowspan)) in rows.iter().enumerate() {
            //print!(" {:?} ", content);

            
            let mut vec_parsed_lecutre_info: Option<Vec<(String, String, String, String)>> = Some(Vec::new()); 
            if !content.is_empty() && i > 0 && j > 0 {
                match parse_lecture_info(&content) {
                    Some(parsed_infos) => {
                        for (name, prof, location, discription) in parsed_infos {
                            lecture_in_struct.push(Lecture {
                                name,
                                location,
                                description: Some(discription),
                                start: None,
                                end: None,
                                hours: None,
                            });
                        }
                    }
                    None => {
                        // Optional: Logging oder Fehlerbehandlung
                        println!("Keine gültigen Vorlesungsinformationen gefunden in: '{}'", content);
                    }
                }
            }

        }
        //println!("\n")
    }
    

    Ok(lecture_in_struct)
}

fn parse_lecture_info(input: &str) -> Option<Vec<(String, String, String, String)>> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    //println!("Input: {}", input);

    if parts.len() < 3 {
        return None; // nicht genug Infos
    }



    let re_location = Regex::new(r"BT(\d+)-(\d+)").ok()?;
    let (loc_vec, loc_pos_vec): (Vec<_>, Vec<_>) = re_location
        .find_iter(input)
        .map(|m| (m.as_str().to_string(), m.end()))
        .unzip();

    let re_semester_week = Regex::new(r"ab(\d+) SW").ok()?;
    let pos_sw_vec: Vec<usize> = re_semester_week
        .find_iter(input)
        .map(|m| m.end())
        .collect();


    let mut input_split: Vec<String> = {
        let input = input; // explizit referenzieren
        loc_pos_vec
            .iter()
            .copied()
            .chain(std::iter::once(input.len()))
            .scan(0, move |start, end| {
                let part = input[*start..end].to_string();
                *start = end;
                Some(part)
            })
            .collect()
};

            
    //println!(" String aufgeteilt:{:?}", input_split);

    for i in 0..input_split.len() {
        let current = &input_split[i];

        if re_location.find(current).is_none() && i > 0 {
            let combined = format!("{} {}", input_split[i - 1], current);
            input_split[i - 1] = combined;
            input_split.remove(i);
            break; // oder pass an, wenn du mehrere zusammenfassen willst
        }
    }

    //println!(" String aufgeteilt: {:?}", input_split);

    let mut output_vec: Vec<(String, String, String, String)> = Vec::new();

    for classes in input_split {
        let location = re_location.find(input)?.as_str().to_string();

    //Pattern für Prof erstellen:
        let le_prof = Regex::new(r"[A-Z][a-z]{2,3}").ok()?;
        let prof = le_prof.find(input)?.as_str().to_string();

    /* um die Namen zu filtern, werden mehrere Regex gebraucht, da es keine einheitliche Struktur, bei den namen gibt:
    PureThreeLetter	->          Genau drei Großbuchstaben, keine Leerzeichen oder Sonderzeichen
    ThreeLetterWithSuffix ->    Drei Großbuchstaben + Leerzeichen + L oder U (vermutlich Labor/Übung)
    DashCode ->                 Abkürzungen mit einem Bindestrich und einer Zahl
    TwoLetterWithSpace ->       Zwei Buchstaben (oder mehr) + Leerzeichen + Buchstabe oder Zahl
    WithSuffixPOrM ->           Abkürzungen mit zusätzlichem Buchstaben wie P oder M für Praxis/Modul etc.
    Other ->                    Abkürzungen, die nicht klar in eine der obigen Kategorien passen
    */

        let patterns = [
            r"\b[A-Z]{3} [LU]\b",       // RTT L, TM A U
            r"\b[A-Z]{3}\.?" ,            // RTT, WZM
            r"[A-Z]+-\d",               // MAT-1
            r"[A-Z]+ \d",               // TM A, Kon 3
            r"[A-Z]+ [PM]",             // Mkon P
            r"\b[A-Z]{4,}\b",           // ILOG, FLUIDT (ohne Satzzeichen)
            r"\b[A-Z]{4,} L\b",         // FLUIDT L (ohne » oder Punkt)
            r"»?[A-Z]{4,}(?: L)?\.?",   // robuste Version für FLUIDT, FLUIDT L, ILOG, mit » und/oder Punkt
        ];

        let vec_name_regex: Vec<Regex> = patterns
            .iter()
            .map(|pat| Regex::new(pat).ok())
            .collect::<Option<Vec<_>>>()?;

        let mut name: String = String::new();

        for re_name in vec_name_regex {
            name = re_name.find(input)?.as_str().to_string();
            if !name.is_empty() {
            //println!("Vorlesung mit {:?} gefunden", re_name);
                break;
            }
            
        
        }

        let discription: String = String::new();

        

        output_vec.push((name, prof, location, discription));
        println!("Infos vor dem Matchen: {:?}", output_vec);
    }

   

    Some(output_vec)
}

/*
Um die weitere Verarbeitung in dem HAWHHCalendarBot Cli zu vereinfachen, währe ein 
Ausgabe in dem Kalenderformat .ics nützlich
*/
pub fn lecture_plan_to_ics(){}