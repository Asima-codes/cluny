# 📁 Cluny – Duplicate File Detector in Rust

Cluny is a Rust command-line tool for detecting duplicate files by content. It helps you **identify**, **quarantine**, **delete**, and **restore** duplicates with a simple interface.

---

## 🚀 Features

- Detects duplicates using SHA-256 hashing
- Quarantines duplicates to `.quarantine_files`
- Permanently deletes quarantined files
- Restores quarantined files to their original location

---

## 🧰 Dependencies

Add these crates to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
walkdir = "2.4"
sha2 = "0.10"
```

---

## 🗂️ Project Structure

```
cluny/
├── src/
│   ├── main.rs         # CLI & entry point
│   ├── compare.rs      # Duplicate detection
│   └── quarantine.rs   # Quarantine, delete, restore logic
├── Cargo.toml
```

---

## 🧪 How It Works

1. Recursively scans the target directory
2. Computes SHA-256 hash for each file
3. Groups files with identical hashes as duplicates
4. Lets you view, quarantine, delete, or restore duplicates

---

## 💻 Usage

### 1. Build and Run

```bash
cargo build --release
./target/release/cluny <COMMAND> [OPTIONS]
```

### 2. Commands

- **Show duplicates**
    ```bash
    cluny same --path <directory>
    ```
    Lists duplicate file groups.

- **Quarantine duplicates**
    ```bash
    cluny quarantine --path <directory>
    ```
    Moves one duplicate per group to `.quarantine_files`.

- **Delete quarantined files**
    ```bash
    cluny delete
    ```

- **Restore quarantined files**
    ```bash
    cluny restore
    ```

### 3. Example

```bash
cluny same --path ./test_folder
cluny quarantine --path ./test_folder
cluny delete
cluny restore
```

---

## ⚠️ Notes

- Only one file per duplicate group is quarantined
- Quarantine folder: `.quarantine_files` inside the target directory
- Deletion is permanent—use with caution

---

## 🛠️ Tech Highlights

- CLI parsing: `clap`
- Directory traversal: `walkdir`
- Hashing: `sha2`
- Modular Rust structure