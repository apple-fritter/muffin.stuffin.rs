# mff-bookmark-egress.rs
A reinterpretation of my script [mff-bookmark-egress.sh](https://github.com/apple-fritter/mff-bookmark-egress.sh)
 in Rust which provides more flexibility and user-friendliness, as it allows the user to specify the file paths and offers search and default options. It also uses a more efficient and robust approach by using Rust's built-in `SQLite` library and `standard` library file I/O. Additionally, the Rust implementation provides better error handling and informative error messages to the user.

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

## Requirements: 
* Rust programming language: Rust must be installed on the system where the program will be run. Rust can be installed by following the instructions on the official [Rust website](https://www.rust-lang.org/tools/install).

* `SQLite` library: The SQLite library must be available on the system where the program will be run. On Linux-based systems, SQLite can usually be installed using the system package manager. On Windows, SQLite can be downloaded from the [SQLite website](https://www.sqlite.org/download.html).

* `SQLite` Rust crate: The Rust implementation uses the rusqlite crate, which provides a safe and convenient way to interact with SQLite databases in Rust. This crate can be added to the Rust project's dependencies by including it in the project's `Cargo.toml` file.

* `Standard` library: The Rust standard library provides built-in functionality for performing file I/O, string manipulation, and error handling, which are all used in the implementation of the program.

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

## License

These files released under the [MIT License](LICENSE).
