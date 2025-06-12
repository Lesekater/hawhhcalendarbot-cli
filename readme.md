# HAW Hamburg Calendar Bot CLI

This project is part of the Rust WP SoSe2025 course. The goal is to create a command-line interface (CLI) for the Telegram HAW Hamburg Calendar Bot ([GitHub Repository](https://github.com/HAWHHCalendarBot)).

## Project Goals

### Main Goal
- Display and filter Mensa meals by different locations and days, similar to the Telegram bot.

### Goal
#### Jun 03. (Error Handling, _Serde_ Json)
- Parsing Json Mensa & Filtering
- Config Mensa
#### Jun 24. (Traits & Generics)
- Implement Traits for Mensa
- Parse Cal Entries from Informatik, Elektrotechnik
- Add Config for Cal Entries (My Modules...)
#### Jul 08. (Multi-Threadig, Enums & Pattern Matching)
- Filter and show calendar entries by own config
- Add Webscaper for Mechatronic Entries
- Add Multithreading

### Soft Goals
1. Add functionality to parse calendar data for Mechatronik students. This may involve extending the downloader ([GitHub Repository](https://github.com/HAWHHCalendarBot/downloader)) to handle PDF parsing for schedules not available as ICS files.
2. Filter calendar entries and display them via the CLI.
3. Perform collision checks for calendar entries to help select non-conflicting courses.

## Current Status
- Basic client functionality is implemented.
- Mensa data can be downloaded, parsed, and displayed.
- Filtering of Mensa data based on the configuration is possible.
- Configuration can be read using Serde (custom JSON reader still TODO).
- Write commands for the configuration are still TODO.
- The configuration is used when fetching Mensa data.

## Team Members

- Niclas Waßmann
- Birger Müller
- Elias Wernicke

## Notes
- For Mechatronik schedules, some data is only available in PDF format, requiring a PDF scraper to extract relevant information.
- Informatik schedules are available as ICS files, simplifying parsing.
- Elektrotechnik schedules also provide ICS files, but other departments may require additional parsing efforts.

- instead of secondary mensas we will implment aliases wich replace the add and remove commands with alias. Addritial Mensas can be accesed via a flag --mensa \<alias>
