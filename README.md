<h2 align="start">Genesis 🌱</h1>
<h3 align="start">Genesis is a powerful, flexible, and user-friendly project starter tool designed for Go and Rust developers. It streamlines the initial setup process, allowing developers to focus on what truly matters: building great applications and solving business problems.</h3>

<p align="start">
    <img src="./genesis.png" alt="genesis" />
</p>

## Get Started

```sh
cargo install genesis_rs
genesis -h
```

## 🚀 Why Genesis?

When developing software, setting up a new project can be a time-consuming and oftenly a frustrating process. Genesis solves this problem by:

- **Quick Setup and Interactive CLI**: Quickly initialize Go and Rust projects using your CLI for easy project configuration
- **Language Support**: Support for Go and Rust, with room for expansion
- **Customizable Paths**: Flexibly set project locations
- **Automatic Dependency Management**: Run `go mod tidy` for Go and `cargo build` for Rust projects
- **Cross-Platform**: Works on Windows, macOS, and Linux-based systems

By handling the initial setup, Genesis allows developers to immediately dive into application core functionality and business logic, significantly reducing time-to-market for new ideas.s

## 🛠 Installation

Install Genesis using Cargo, the Rust package manager:

```bash
cargo install genesis_rs
```

## 📘 Usage

Genesis offers both interactive and non-interactive modes to suit your workflow:

### Interactive Mode

Simply run:

```bash
genesis run
```

You can update to the latest version of genesis by running:
```bash
genesis update
```

Follow the prompts to select your project language and specify the project name.

### Non-Interactive Mode

Specify the language and path directly:

```bash
genesis run --language <LANG> --path /path/to/your/project
```

For example:

```bash
genesis run --language rust --path /path/to/your/project
```

### Available Commands

- `genesis`: Display version and available commands
- `genesis run`: Start the interactive project setup
- `genesis run --language <LANG> --path <PATH>`: Run with specific language and path
- `genesis --help`: Show the help message with all available options

<!-- ## 🤝 Contributing

We welcome contributions to Genesis! Whether it's adding new features, improving documentation, or reporting bugs, your help is appreciated. Please feel free to:

1. Submit issues
2. Fork the repository
3. Send pull requests

Check out our [Contributing Guidelines](CONTRIBUTING.md) for more details. -->

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 📞 Contact

If you have any questions or need support, feel free to:

- Open an issue on GitHub
- Reach out to the maintainer: [Thembinkosi Mkhonta](https://github.com/ThembinkosiThemba)
