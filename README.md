# HICSS Smart Search

With Smart Search, users can search for papers from the chat UI of AI providers such as Claude or ChatGPT. Integration with these generative AI providers is achieved by the Model Context Protocol ([MCP](https://modelcontextprotocol.io/docs/getting-started/intro)) standard.

## Requirements
Make sure you have Rust installed ([documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)):

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
      
## Checkout the code

```sh
git clone git@github.com:zaira-bibi/hicss-smart-search.git        # ssh
cd hicss-smart-search
```

Build the application:

    cargo build --release

The compiled executable will be in `target/release`. Run it with:

    cargo run --release --

### Options

#### Transport mode: server-sent events (SSE) / stdio

Smart Search MCP server can run in two modes, selectable via the `--mode` flag:

| Mode    | Description                                          |
| ------- | ---------------------------------------------------- |
| `stdio` | Communicates via standard input/output streams.      |
| `sse`   | Starts an HTTP server with Server-Sent Events.       |

The default is `sse` on host http://0.0.0.0:8080.

This starts an SSE server on the given host and port with:

* `GET /` for establishing the event stream.
* `POST /message` for sending messages.

The `stdio` transport mode is appropriate for running with Claude locally:

    ./sparkth --mode=stdio


Any CLI option supported can be appended to

```
cargo run --release --
```

## Usage

### Local usage
Add the `smart-search-mcp` (or `smart-search-mcp.exe`) executable as an external tool to Claude Desktop by editing the Claude configuration file:

    # macOS
    ~/Library/Application\ Support/Claude/claude_desktop_config.json
    # Windows
    %APPDATA%\Claude\claude_desktop_config.json
    # Linux
    ~/.config/Claude/claude_desktop_config.json


Add the MCP server, such that your configuration looks like the following:

    {
      "mcpServers": {
        "HICSS stdio": {
          "command": "/<PATH TO HICSS SMART SEARCH REPOSITORY>/target/release/smart-search-mcp",
          "args": ["--mode=stdio"],
        }
      }
    }

Restart Claude Desktop. Ensure that the "HICSS stdio" tools appear in the "Search and tools" menu. Then start a new chat and prompt Claude:

> Use HICSS to search papers by John Doe

