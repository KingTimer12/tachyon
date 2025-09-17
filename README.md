# Tachyon

[![npm version](https://badge.fury.io/js/tachyon.svg)](https://badge.fury.io/js/tachyon)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance HTTP server framework for Node.js, built with Rust and powered by [napi-rs](https://napi.rs). Tachyon leverages the speed of Rust's [Hyper](https://hyper.rs/) HTTP library while providing a familiar Express-like API for Node.js developers.

## Features

- 🚀 **High Performance**: Built with Rust and Hyper for maximum throughput
- 🔄 **Node.js Integration**: Seamless integration with Node.js through NAPI-RS
- 🛠 **Express-like API**: Familiar routing and middleware patterns
- 🎯 **TypeScript Support**: Full TypeScript definitions included
- 🔧 **Cross-platform**: Supports multiple architectures and operating systems
- ⚡ **Zero Dependencies**: Minimal runtime overhead

## Installation

```bash
npm install tachyon
# or
yarn add tachyon
# or
pnpm add tachyon
```

## Quick Start

```typescript
import { tachyon } from 'tachyon'

const app = tachyon()

// Define a route
app.get('/', (req, res) => {
  // Handle request
})

// Start the server
app.listen(3000)
console.log('Server running on http://localhost:3000')
```

## API Reference

### Creating a Server

```typescript
import { tachyon } from 'tachyon'

const app = tachyon()
```

### Routing

#### GET Routes

```typescript
app.get('/path', (request: TachyonRequest, response: TachyonResponse) => {
  // Handle GET request
})
```

#### Listing Routes

```typescript
const routes = app.routes()
console.log(routes) // ['GET /path', 'GET /another']
```

### Server Methods

#### listen(port: number)

Start the HTTP server on the specified port.

```typescript
await app.listen(3000)
```

## Examples

### Basic Server

```typescript
import { tachyon } from 'tachyon'

const app = tachyon()

app.get('/', (req, res) => {
  // Return "Hello, World!"
})

app.get('/api/health', (req, res) => {
  // Health check endpoint
})

app.listen(3000)
```

### Route Management

```typescript
import { tachyon } from 'tachyon'

const app = tachyon()

// Add multiple routes
app.get('/', (req, res) => {})
app.get('/users', (req, res) => {})
app.get('/posts', (req, res) => {})

// List all registered routes
console.log(app.routes())
// Output: ['GET /', 'GET /users', 'GET /posts']
```

## Architecture

Tachyon is built on top of:

- **Rust**: For high-performance HTTP handling
- **Hyper**: Rust's fast HTTP library
- **Tokio**: Asynchronous runtime for Rust
- **NAPI-RS**: For seamless Node.js integration
- **DashMap**: Thread-safe concurrent hash map for route storage

## Performance

Tachyon is designed for high-performance scenarios. The Rust backend handles HTTP parsing, routing, and response generation, while maintaining a simple JavaScript API.

### Benchmarks

Run the included benchmarks:

```bash
npm run bench
```

## Development

### Prerequisites

- Node.js 12.22.0+ or 14.17.0+ or 15.12.0+ or 16.0.0+
- Rust 1.60+
- Cargo

### Building from Source

```bash
# Clone the repository
git clone https://github.com/KingTimer12/tachyon.git
cd tachyon

# Install dependencies
npm install

# Build the native module
npm run build

# Run tests
npm test
```

### Available Scripts

- `npm run build` - Build the native module in release mode
- `npm run build:debug` - Build in debug mode
- `npm run test` - Run the test suite
- `npm run bench` - Run performance benchmarks
- `npm run format` - Format code (Rust, TypeScript, TOML)
- `npm run lint` - Lint code

### Project Structure

```
tachyon/
├── src/                    # Rust source code
│   ├── core/              # Core functionality
│   │   ├── tachyon.rs     # Main server implementation
│   │   └── router.rs      # Request/Response types
│   ├── utils.rs           # Utility functions
│   ├── server.rs          # Server factory
│   └── lib.rs             # Library entry point
├── __test__/              # Test files
├── example/               # Example usage
├── benchmark/             # Performance benchmarks
├── index.d.ts             # TypeScript definitions
├── index.js               # JavaScript entry point
└── package.json
```

## Supported Platforms

Tachyon supports the following platforms:

- Windows (x64, x86, ARM64)
- macOS (x64, ARM64)
- Linux (x64, x86, musl)
- FreeBSD (x64)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Guidelines

1. Follow the existing code style
2. Add tests for new features
3. Update documentation as needed
4. Ensure all tests pass
5. Run formatting and linting tools

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Related Projects

- [napi-rs](https://napi.rs) - Framework for building Node.js addons in Rust
- [Hyper](https://hyper.rs) - Fast HTTP library for Rust
- [Tokio](https://tokio.rs) - Asynchronous runtime for Rust

## Acknowledgments

- Built with [napi-rs](https://napi.rs)
- Inspired by Express.js and Fastify frameworks
- Powered by the Rust ecosystem

---

For more information, examples, and advanced usage, please visit our [documentation](https://github.com/kingtimer12/tachyon).
