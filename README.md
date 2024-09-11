# **depScanner**

**depScanner** is a lightweight and efficient dependency scanner tool that analyzes lock files (e.g., `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml`, `bun.lockb`) and `package.json` for vulnerabilities. Built with Rust, it helps developers ensure their dependencies are secure and up-to-date.

---

## **Features**

- Scans lock files for exact dependency versions.
- Supports multiple formats:
  - `package-lock.json`
  - `yarn.lock`
  - `pnpm-lock.yaml`
  - `bun.lockb`
  - Fallback to `package.json`.
- Identifies known vulnerabilities using public advisory APIs.
- Provides a clean, CLI-friendly vulnerability report.
- Fast and memory-efficient, powered by Rust.

---

## **Getting Started**

To use **depScanner**, choose one of the following methods:

### **Using Rust Locally**
If you want to run depScanner natively on your machine, follow these steps:

1. Ensure you have [Rust](https://www.rust-lang.org/) installed.

2. Clone the repository:

   ```bash
   git clone https://github.com/your-username/depScanner.git
   cd depScanner
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

4. Run the binary:

   ```bash
   ./target/release/depScanner <path-to-lockfile-or-package.json>
   ```

### **Using Docker (Recommended)**
For a hassle-free setup, use the Docker image to run depScanner:

1. Build the Docker image:

   ```bash
   docker build -t dep-scanner .
   ```

2. Run the Docker container:

   ```bash
   docker run --rm -v $(pwd):/app dep-scanner ./depScanner /app/<lock-file-or-package.json>
   ```

   Replace `<lock-file-or-package.json>` with the path to your dependency file.

---

## **Usage**

### **Basic Command**

Scan a lock file or `package.json`:

```bash
./depScanner <path-to-lockfile-or-package.json>
```

### **Examples**

1. Scan a `package-lock.json` file:

   ```bash
   ./depScanner ./package-lock.json
   ```

2. Scan a `yarn.lock` file:

   ```bash
   ./depScanner ./yarn.lock
   ```

3. Scan a `pnpm-lock.yaml` file:

   ```bash
   ./depScanner ./pnpm-lock.yaml
   ```

4. Fallback to `package.json` if lock files are unavailable:

   ```bash
   ./depScanner ./package.json
   ```

5. Using Docker to scan a file:

   ```bash
   docker run --rm -v $(pwd):/app dep-scanner ./depScanner /app/package-lock.json
   ```

---

## **Output**

The CLI displays a vulnerability report like this:

```plaintext
Dependency Vulnerability Report
Package                        Version         Issues
--------------------------------------------------------------------------------
lodash                         4.17.20        Prototype Pollution
express                        4.17.1         XSS Vulnerability
chalk                          2.4.2          No issues found
```

- **Red**: Vulnerable dependencies.
- **Green**: No vulnerabilities found.

---

## **Supported Formats**

- `package-lock.json` (npm)
- `yarn.lock` (Yarn)
- `pnpm-lock.yaml` (pnpm)
- `bun.lockb` (Bun)
- `package.json` (Fallback for declared dependencies)

---

## **License**

This project is licensed under the [MIT License](./LICENSE).

---

## **Acknowledgments**

- Built with ❤️ using Rust.
- Inspired by the need for secure dependency management.

---