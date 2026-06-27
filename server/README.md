# Rustic Vault

This project is a simple file server written in Rust, using the Rocket framework. It allows users to upload files through a form and serves static files from a specified directory. The server handles different content types for images and uses environment variables for configuration.

## Features

- **Secure API Access:** Protected endpoints with API key authentication.
- **File Upload & Persistence:** Supports uploading various file types and persisting them to a configurable directory.
- **Dynamic File Serving:** Serve files dynamically from the server with support for a wide range of file types.
- **Cross-Origin Resource Sharing (CORS):** Configured CORS to allow web applications to interact with the server from different origins.
- **Environment Variables:** Use environment variables for easy configuration of API keys, server URL, and file storage paths.

## Prerequisites

- Rust and Cargo (latest stable version recommended)
- [Rocket](https://rocket.rs) (ensure compatibility with your Rust version)
- `dotenv` for environment variable management

## Setup

1. **Clone the repository:**

    ```sh
    git clone https://github.com/brendandonahue/rust-file-server.git
    cd rust-file-server
    ```

2. **Set up your environment variables:**

    Create a `.env` file in the root of your project and specify the `GLOBAL_PATH` variable, which determines the directory for uploads and static file serving, as well as the `API_KEY` you'd like to use to secure server from unauthorized requests:

    ```
    GLOBAL_PATH=/path/to/your/uploads
    API_KEY=super_secret_key
    RUST_FILE_SERVER_URL=your_server_url
    ```

3. **Install dependencies:**

    Ensure you are using a nightly Rust toolchain, as Rocket requires it. You can override the toolchain for your project directory by running:

    ```sh
    rustup override set nightly
    ```

    Then, install the project dependencies:

    ```sh
    cargo build
    ```

## Usage
### Uploading Files
    To upload files, send a POST request to /upload/form with the X-API-KEY header and a form-data body containing the file.

### Accessing Files
    Access uploaded files by navigating to /files/{file_name}. The server dynamically serves the files stored in the configured directory.

## Running the Server

To start the server, run:

```sh
cargo run
```

## Build a Release

To build a release to use in production environment:

```sh
cargo build --release