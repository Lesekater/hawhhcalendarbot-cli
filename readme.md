# HAW Hamburg Calendar Bot CLI

This project is part of the Rust WP SoSe2025 course. The goal is to create a command-line interface (CLI) for the Telegram HAW Hamburg Calendar Bot ([GitHub Repository](https://github.com/HAWHHCalendarBot)).

## Installation

To install the CLI tool directly from GitHub, run:

```bash
cargo install --git https://github.com/Lesekater/hawhhcalendarbot-cli.git
```

After installation, you can use the tool with the `hawhhcalendarbot-cli` command.

## Usage

### Mensa Commands

The CLI provides comprehensive mensa functionality to check meal plans and configure settings.

#### Basic Commands

```bash
# Show today's mensa menu (requires primary mensa to be set)
hawhhcalendarbot-cli mensa

# Show today's menu explicitly
hawhhcalendarbot-cli mensa today

# Show tomorrow's menu
hawhhcalendarbot-cli mensa tomorrow

# Show menu for a specific date (DD.MM.YYYY format)
hawhhcalendarbot-cli mensa date 15.07.2025
```

#### Configuration

Before using mensa commands, you need to configure your primary mensa:

```bash
# Set primary mensa
hawhhcalendarbot-cli mensa settings primary "Mensa Berliner Tor"

# Add additional mensas
hawhhcalendarbot-cli mensa settings add "Mensa Armgartstraße"
hawhhcalendarbot-cli mensa settings add "Mensa Finkenau"

# List all configured mensas
hawhhcalendarbot-cli mensa settings list

# Remove a mensa
hawhhcalendarbot-cli mensa settings remove "Mensa Finkenau"

# Set your occupation (affects pricing)
hawhhcalendarbot-cli mensa settings occupation student
hawhhcalendarbot-cli mensa settings occupation employee
hawhhcalendarbot-cli mensa settings occupation guest
```

#### Dietary Filters

Configure dietary preferences to filter meal options:

```bash
# Add dietary filters
hawhhcalendarbot-cli mensa settings extras vegan
hawhhcalendarbot-cli mensa settings extras vegetarian
hawhhcalendarbot-cli mensa settings extras lactose-free

# Remove specific dietary requirements
hawhhcalendarbot-cli mensa settings extras no-alcohol
hawhhcalendarbot-cli mensa settings extras no-beef
```

#### Advanced Options

```bash
# Show menu from additional mensas using their index
hawhhcalendarbot-cli mensa --number 1  # Shows first additional mensa
hawhhcalendarbot-cli mensa tomorrow --number 2  # Shows second additional mensa for tomorrow

# Get output in JSON format
hawhhcalendarbot-cli mensa --json

# Force update mensa data
hawhhcalendarbot-cli mensa update

# Show config file location
hawhhcalendarbot-cli mensa settings config

# Delete configuration
hawhhcalendarbot-cli mensa settings delete
```

### Events Commands (Coming Soon)

```bash
# List modules for a specific department (with filtering)
hawhhcalendarbot-cli events list-modules informatik --filter "ad"

# Add event to config
hawhhcalendarbot-cli events add bai3-ad informatik

# List all events for a specific date
hawhhcalendarbot-cli events get 2025-07-04
```

## Project Goals

### Completed Goals
#### Jun 03. (Error Handling, _Serde_ Json)
- ✅ Parsing Json Mensa & Filtering
- ✅ Config Mensa

#### Jun 24. (Traits & Generics)
- ✅ Implement Traits for Mensa
- 🔄 Parse Cal Entries from Informatik, Elektrotechnik
- 🔄 Add Config for Cal Entries (My Modules...)

#### Jul 08. (Multi-Threading, Enums & Pattern Matching)
- 🔄 Filter and show calendar entries by own config
- 🔄 Add Webscraper for Mechatronic Entries
- ✅ Add Multithreading
  - ✅ Add Channels

### Soft Goals
1. Add functionality to parse calendar data for Mechatronik students. This may involve extending the downloader ([GitHub Repository](https://github.com/HAWHHCalendarBot/downloader)) to handle PDF parsing for schedules not available as ICS files.
2. Filter calendar entries and display them via the CLI.
3. Perform collision checks for calendar entries to help select non-conflicting courses.

## Team Members

- Niclas Waßmann
- Birger Müller
- Elias Wernicke

## Technical Notes

- For Mechatronik schedules, some data is only available in PDF format, requiring a PDF scraper to extract relevant information.
- Informatik schedules are available as ICS files, simplifying parsing.
- Elektrotechnik schedules also provide ICS files, but other departments may require additional parsing efforts.
- The tool uses local caching to improve performance and reduce API calls.
- Multi-threading is implemented for efficient data filtering and processing.

## Contributing

This project is part of a university course. Contributions are welcome through pull requests on the GitHub repository.