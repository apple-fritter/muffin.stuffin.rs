use std::io::{self, Write};
use std::path::{Path, PathBuf};
use rusqlite::{Connection, Result};
use url::Url;

fn main() -> Result<()> {
    let places_file_path = get_places_file_path()?;
    let output_file_path = get_output_file_path()?;
    let conn = Connection::open(&places_file_path)?;

    let mut stmt = conn.prepare(
        "SELECT moz_bookmarks.title, moz_places.url FROM moz_bookmarks
        JOIN moz_places ON moz_bookmarks.fk = moz_places.id
        WHERE moz_bookmarks.type = 1 ORDER BY moz_bookmarks.position",
    )?;

    let bookmarks = stmt
        .query_map([], |row| {
            let title: String = row.get(0)?;
            let url_str: String = row.get(1)?;
            let url = Url::parse(&url_str)?;
            Ok((title, url))
        })?
        .collect::<Result<Vec<(String, Url)>>>()?;

    let mut output_file = std::fs::File::create(&output_file_path)?;
    writeln!(&mut output_file, "<html><body><ul>")?;
    for (title, url) in bookmarks {
        writeln!(
            &mut output_file,
            "<li><a href=\"{}\">{}</a></li>",
            url.as_str(),
            title.escape_default()
        )?;
    }
    writeln!(&mut output_file, "</ul></body></html>")?;

    Ok(())
}

fn get_places_file_path() -> io::Result<PathBuf> {
    loop {
        print!("Enter the path to your Firefox places.sqlite file: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let places_file_path = Path::new(input.trim());
        if places_file_path.exists() {
            return Ok(places_file_path.to_path_buf());
        } else {
            println!("The file {} does not exist.", places_file_path.display());
        }
    }
}

fn get_output_file_path() -> io::Result<PathBuf> {
    loop {
        print!("Enter the path to the output HTML file (default is ~/Desktop/bookmarks.html): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let output_file_path = if input.trim().is_empty() {
            dirs::desktop_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("bookmarks.html")
        } else {
            Path::new(input.trim()).to_path_buf()
        };
        if let Some(parent_dir) = output_file_path.parent() {
            if parent_dir.exists() {
                return Ok(output_file_path);
            } else {
                println!("The directory {} does not exist.", parent_dir.display());
            }
        } else {
            return Ok(output_file_path);
        }
    }
}
