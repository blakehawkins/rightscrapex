# rightscrapex

`rightscrapex` is a command-line tool for scraping property data from rightmove.co.uk. It takes Rightmove property URLs from standard input, scrapes the pages, and outputs the data as JSON.

## Usage

To use `rightscrapex`, you can pipe a URL to the `cargo run` command.

### Basic Usage

```bash
echo "https://www.rightmove.co.uk/properties/100454543#/" | cargo run -- --json
```

This will output a JSON object containing the scraped data for the given URL.

### Options

- `--floorplan`: Only output data for properties that have a floorplan.
- `--json`: Output the data in JSON format.
- `--urls`: Output only the URLs of the properties.

### Examples

#### Get JSON data for a property

```bash
echo "https://www.rightmove.co.uk/properties/100454543#/" | cargo run -- --json
```

#### Get JSON data for a property with a floorplan

```bash
echo "https://www.rightmove.co.uk/properties/100454543#/" | cargo run -- --floorplan --json
```

#### Get the URL for a property

```bash
echo "https://www.rightmove.co.uk/properties/100454543#/" | cargo run -- --urls
```

## Development

This project uses `just` to manage common development tasks.

- `just check`: Check the code for errors.
- `just fmt`: Format the code.
- `just clippy`: Lint the code.
- `just test`: Run the tests.
- `just update`: Update the dependencies.
- `just outdated`: Check for outdated dependencies.
