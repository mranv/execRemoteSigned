<h1 align="center">
<br>
<img src=assets/rust-ferris.gif >
<br>

</h1>

`execRemoteSigned` is a Rust program designed to set the PowerShell execution policy to `RemoteSigned`. This allows local unsigned scripts and remote signed scripts to be executed on a Windows system. The program ensures compatibility with various Windows versions and includes necessary safety checks.

## Features

- **Windows Version Detection**: Uses the `GetVersionExW` API to detect Windows versions, ensuring compatibility with Windows XP, 7, 8, 8.1, 10, and 11.
- **Registry Operations**: Interacts with the Windows Registry to set the PowerShell execution policy.
- **Error Handling**: Includes error handling for potential failures, such as inability to open/create the registry key or set the execution policy value.
- **Administrative Privileges Required**: The binary must be run with administrative privileges to modify the registry.

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/mranv/execRemoteSigned.git
   cd execRemoteSigned
   ```

2. **Build the Project**:
   Ensure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build --release
   ```

## Usage

1. **Run the Program**:
   Execute the binary with administrative privileges to modify the registry:

   ```bash
   ./target/release/execRemoteSigned
   ```

   This will set the PowerShell execution policy to `RemoteSigned`.

## Contribution

Contributions are welcome! Follow these steps to contribute:

1. **Fork the Repository**:
   Click on the `Fork` button at the top right of the repository page.

2. **Clone Your Fork**:

   ```bash
   git clone https://github.com/your-username/execRemoteSigned.git
   cd execRemoteSigned
   ```

3. **Create a Branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make Changes and Commit**:

   ```bash
   git commit -m "Add your feature description"
   ```

5. **Push to Your Fork**:

   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request**:
   Open a pull request from your fork's branch to the main repository.

## Contact

For any issues or questions, please open an issue on GitHub or contact the repository owner.

---

© 2024 mranv. All rights reserved.
