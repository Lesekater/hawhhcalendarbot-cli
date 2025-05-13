# HAW Hamburg Calendar Bot CLI

This project is part of the Rust WP SoSe2025 course. The goal is to create a command-line interface (CLI) for the Telegram HAW Hamburg Calendar Bot ([GitHub Repository](https://github.com/HAWHHCalendarBot)).

## Project Goals

### Main Goal
- Display and filter Mensa meals by different locations and days, similar to the Telegram bot.

### Soft Goals
1. Add functionality to parse calendar data for Mechatronik students. This may involve extending the downloader ([GitHub Repository](https://github.com/HAWHHCalendarBot/downloader)) to handle PDF parsing for schedules not available as ICS files.
2. Filter calendar entries and display them via the CLI.
3. Perform collision checks for calendar entries to help select non-conflicting courses.

## Team Members

- Niclas Waßmann
- Birger Müller
- Jonas Addicks
- Elias Wernicke

## Notes
- For Mechatronik schedules, some data is only available in PDF format, requiring a PDF scraper to extract relevant information.
- Informatik schedules are available as ICS files, simplifying parsing.
- Elektrotechnik schedules also provide ICS files, but other departments may require additional parsing efforts.