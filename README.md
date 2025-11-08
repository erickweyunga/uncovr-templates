# Uncovr Templates

Official templates for the Uncovr web framework.

## Available Templates

### Default Template

A basic web application template with:
- Health check API endpoint
- Index page with Tera templating
- Hot reload in development mode
- Static file serving
- Environment-based configuration

## Usage

```bash
# Create a new app with the default template
unc create-app my-app

# Create from a specific template
unc create-app my-app --template default

# Create from a custom repository
unc create-app my-app --repo username/repo-name --template custom-template
```

## Creating Your Own Templates

Templates are simple project structures with placeholder variables:

- `{{project_name}}` - Will be replaced with the project name

### Template Structure

```
template-name/
├── Cargo.toml
├── src/
├── templates/
└── public/
```

## Contributing

Feel free to contribute new templates by submitting a pull request!

## License

MIT
