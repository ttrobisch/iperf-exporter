# Iperf exporter

This Rust application serves as an endpoint for running `iperf3` tests against a given target server. The application is built using the Actix Web framework.

## Features

- **Endpoint for Iperf3 Tests**: The `/probe` endpoint allows users to run `iperf3` tests against a specified target server. Tests are run serially to prevent simultaneous tests for accurate results.

- **Docker Support**: The application can be containerized using Docker, allowing for easy deployment.

## Prerequisites

- Iperf3 (if you plan to run tests)
- Docker (if you plan to run the application in a container)

### Installing Iperf3

- **Archlinux**: `sudo pacman -S iperf3`
- **Ubuntu**: `sudo apt install iperf3`
- **MacOS**: `brew install iperf3`
- **Windows**: `choco install iperf3`
- **Other**: [Download](https://iperf.fr/iperf-download.php) and install the binary for your platform.

## Getting Started

1. **Clone the Repository**

```bash
git clone https://github.com/ttrobisch/iperf-exporter.git
cd iperf-exporter
```

2. **Run Locally**

Compile and run the application:

```bash
cargo run
```

The server will start and listen on `0.0.0.0:3030`.

3. **Using Docker**

Run the Docker container:

```bash
docker run -d -p 3030:3030 ttrobisch/iperf-exporter
```

## API Usage

To run an `iperf3` test:

```
GET /probe?target=YOUR_IPERF_SERVER_IP
```

Replace `YOUR_IPERF_SERVER_IP` with the IP address of your `iperf3` server.

## Usage with prometheus

To use this application with prometheus, you can use the following scrape configuration:

```
TODO
```

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

[MIT](LICENSE)
