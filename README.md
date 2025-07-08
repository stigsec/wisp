# wisp
**wisp** is a simple, lightweight and fast Rust-based HTTP file server. It is intended as a minimal replacement for Python's `http.server`.

---

## Features

- Serve static files from the current directory
- Default port 8000, configurable with `-p` flag
- Basic logging of HTTP requests
- Supports `GET` requests for file downloads
- Supports `POST` request to root `/` to list files (files only)
- Minimal dependencies, single binary

---

## Installation

### Build from source

1. Clone the repo:

   ```bash
   git clone https://github.com/stigsec/wisp.git
   cd wisp
   ```
2. Build with Cargo:
   ```bash
   cargo build --release
   ```
   
---

## Usage

Start serving files from the current direcotry on default port 8000:
```bash
wisp
```
Serve files on a custom port, e.g., 9000:
```bash
wisp -p 9000
```
Display help menu:
```bash
wisp -h
```

---

## Examples
Download a file using `GET`:
```bash
curl http://localhost:8000/example.txt
```
List files with `POST`:
```bash
curl -X POST http://localhost:8000/
```

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE file](LICENSE) for more details.



---

Developed by [stigsec](https://github.com/stigsec).

