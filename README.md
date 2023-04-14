# mff-bookmark-egress.rs
A reinterpretation of my script in Rust which provides more flexibility and user-friendliness, as it allows the user to specify the file paths and offers search and default options. It also uses a more efficient and robust approach by using Rust's built-in `SQLite` library and `standard` library file I/O. Additionally, the Rust implementation provides better error handling and informative error messages to the user.

## Side-by-side comparison
A comparison of the functionality of the Rust implementation and the Bash script:
### Rust implementation:
* Prompts the user for the path to the `places.sqlite` file and the output HTML file
* Allows the user to search for the places.sqlite file from the home directory
* Allows the user to default the output HTML file to `~/Desktop/bookmarks.html`
* Uses Rust's built-in `SQLite` library to connect to the Firefox database file and retrieve bookmark data
* Uses Rust's `standard` library to perform file I/O and write the bookmark data to the output HTML file
* Provides error handling and informative error messages to the user
### Bash script:
* Assumes that the `places.sqlite` file is located in the same directory as the script
* Exports the bookmark data to an HTML file named `bookmarks.html` in the same directory as the script
* Uses `SQLite3` command-line tool to execute SQL queries and retrieve bookmark data
* Uses command-line tools like `sed` and `awk` to process the bookmark data and format it as HTML
* Provides no error handling or informative error messages to the user
