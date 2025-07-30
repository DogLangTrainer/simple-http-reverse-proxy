# simple-http-reverse-proxy


## Usage

This application is designed to be run as a standalone binary. You do not need to have the Rust toolchain installed to use it.

### 1\. Download the Binary

Go to the [**Releases page**](https://github.com/DogLangTrainer/simple-http-reverse-proxy/releases) for this project.

Download the asset that matches your system's architecture. Based on the build configuration, available options include:

  * `simple-http-reverse-proxy-x86_64-unknown-linux-gnu`
  * `simple-http-reverse-proxy-aarch64-unknown-linux-gnu`
  * `simple-http-reverse-proxy-x86_64-unknown-linux-musl` (statically linked)
  * `simple-http-reverse-proxy-aarch64-unknown-linux-musl` (statically linked)

### 2\. Make the Binary Executable

After downloading, you need to grant it permission to execute. It's also convenient to rename it.

```bash
# Example for the x86_64 GNU version
# Rename the file for easier use
mv simple-http-reverse-proxy-x86_64-unknown-linux-gnu simple-proxy

# Make it executable
chmod +x simple-proxy
```

### 3\. Run the Server

You can now run the server directly from your terminal.

**Viewing Help**

To see all available command-line options, run:

```bash
./simple-proxy --help
```

**Running with Default Settings (No prefix, Port 3000)**

```bash
./simple-proxy
```

The server will listen on `127.0.0.1:3000`.

**Running with a Custom Port and Prefix**

```bash
./simple-proxy --port 8080 --prefix proxy
```

This will start the server on port `8080` and use `/proxy` as the URL prefix.

## Examples

Let's assume the target resource is the Rust logo: `https://www.rust-lang.org/static/images/rust-logo-blk.svg`

#### Example 1: Default Mode

**Command:**

```bash
./simple-proxy
```

**Usage:**
Use `curl` to fetch the resource through your proxy. The proxy URL is `http://127.0.0.1:3000/{target_url}`.

```bash
curl -o rust-logo.svg http://127.0.0.1:3000/https://www.rust-lang.org/static/images/rust-logo-blk.svg
```

#### Example 2: Prefixed Mode

**Command:**

```bash
./simple-proxy --port 8080 --prefix proxy
```

**Usage:**
The proxy URL is now `http://127.0.0.1:8080/proxy/{target_url}`.

```bash
curl -o rust-logo.svg http://127.0.0.1:8080/proxy/https://www.rust-lang.org/static/images/rust-logo-blk.svg
```

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/DogLangTrainer/simple-http-reverse-proxy/blob/master/LICENSE) file for details.