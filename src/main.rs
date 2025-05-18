/*
 * Commands:
 * 
 * calendarbot --help / calendarbot help
 * calendarbot --version / calendarbot version
 * ## Mensa
 * calendarbot mensa: shows the mensa menu for today
 * calendarbot mensa tomorrow: shows the mensa menu for tomorrow
 * calendarbot mensa 2023-10-01: shows the mensa menu for the given date
 * calendarbot mensa settings: shows the mensa settings
 *   calendarbot mensa settings <setting> <value>: sets the mensa setting
 *   avalible settings:
 *    - primary: sets the primary mensa
 *    - add: adds a mensa
 *    - remove: removes a mensa
 *    - list: lists all mensas
 *    - occupation: sets the occupation (student, employee, guest)
 *    - extras: sets the extras (vegan, vegetarian, lactose-free, no alcohol, no beef, no fish...)
 *    - show ingredients: shows the ingredients when showing the menu
 * ## Events
 * calendarbot events: shows the selected events
 * calendarbot events list: lists all available events
 * calendarbot events list --filter <filter>: lists all available events with the given filter
 * calendarbot events add <event>: adds the event to the calendar
 * calendarbot events remove <event>: removes the event from the calendar
*/  


fn main() {
    println!("Hello, world!");
}
