# {{project_name}}

A web application built with [Uncovr](https://github.com/erickweyunga/uncovr).

### Installation

```bash
# Install dependencies
cargo build
```

### Running

```bash
# Development mode (with hot reload)
cargo run

# Release mode
cargo run --release
```

The server will start on `http://127.0.0.1:8000` by default.

## Configuration

Configuration can be set via environment variables:

```bash
export APP.NAME="{{project_name}}"
export APP.PORT=8000
export APP.ADDRESS="127.0.0.1"
export ENVIRONMENT="development"
```

Or create a `.env` file:

```env
APP.NAME={{project_name}}
APP.PORT=8000
APP.ADDRESS=127.0.0.1
ENVIRONMENT=development
```

## Project Structure

```
.
├── src/
│   ├── api/           # API endpoints
│   ├── app/           # Web page handlers
│   ├── settings/      # Configuration
│   ├── main.rs        # Application entry point
│   └── routes.rs      # Route definitions
├── templates/         # Tera templates
├── public/           # Static assets
└── Cargo.toml        # Dependencies
```

## Available Endpoints

- `GET /` - Home page
- `GET /health` - Health check endpoint
- `GET /public/*` - Static files

## Development

### Adding a New Page

1. Create a handler in `src/app/`
2. Register it in `src/routes.rs`
3. Create a template in `templates/`

### Adding a New API Endpoint

1. Create an endpoint in `src/api/`
2. Register it in `src/routes.rs`

## Building for Production

```bash
cargo build --release
```

The binary will be available at `target/release/{{project_name}}`.

## License

MIT
