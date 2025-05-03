<!--suppress CheckImageSize -->
<br />

<p align="center">
  <a href="https://github.com/zhaoswen/simx-engine/">
    <img src="runner/resource/SimxIcon.png" alt="Logo" width="80px" height="80px">
  </a>
</p>

<h3 align="center">Simx Quantum Engine</h3>
<p align="center">
    A coding solution tailored for business users
    <br />
    <a href="https://pro.zhaosw.site/docs/engine/quick-start"><strong>Explore the project documentation »</strong></a>
    <br />
    <br />
    <a href="https://github.com/zhaoswen/simx-engine">Designer</a>
     · 
    <a href="https://github.com/zhaoswen/simx-engine/issues">Bugs</a>
     · 
    <a href="https://github.com/zhaoswen/simx-engine/issues">Feature Requests</a>
</p>

## Table of Contents

- [Introduction](#introduction)
- [Design](#design)
- [Concepts](#concepts)
- [Usage](#usage)
- [Compatibility](#compatibility)
- [Use Cases](#use-cases)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Cloning the Repository](#cloning-the-repository)
- [Directory Structure](#directory-structure)
- [Contributors](#contributors)
    - [How to Contribute](#how-to-contribute)
- [Version Control](#version-control)
- [Authors](#authors)
- [About](#about)

## Introduction

The Simx Engine is a streamlined execution engine mainly used for data processing, user operation encapsulation, and
automation tasks. It has no GUI and operates as a command-line tool. It can be installed as a system service or set as
the default executor for blueprint files.

Users design workflows using [Simx Era Design](https://github.com/zhaoswen/simx-era-design) or Melt Studio, while this
project serves as the core engine.

## Design

The design philosophy of Simx is to extract all non-essential features into plugins. These plugins can be added by users
when needed, while the main project retains only the minimal necessary components. The engine focuses on performance and
aims to simplify complex logic, which remains one of its future challenges.

## Concepts

- **Handler**: A handler is an executable operation. It can be built-in, a shell/powershell/cmd/python script, or a
  method from a jar/dll/so file.
- **Blueprint**: A blueprint is a collection of business logic and exists as a file. Once instantiated by the engine, it
  generates one or more flows for execution.
- **Flow**: A flow consists of handlers grouped into logical sequences. Flows can contain multiple nested groups with
  customizable execution order.
- **Component**: A component is a group of handlers that can also contain nested groups.
- **Service**: A service is a continuously running handler, such as a CRON-based trigger or a persistent MySQL service.
- **Extension**: An extension is a collection of handlers and services. Currently, extensions are supported through
  linked libraries, with plans to support integration via Melt Core in the future.

## Usage

Simx Engine can be distributed either as a dynamic library (dll, so, dylib — under the `library` package) or as an
executable binary (`runner` package). When provided as a dynamic library, it can be called from any language supporting
FFI standards. As an executable, it runs a thread listening for incoming commands.

The engine requires standard Simx Blueprint files (BP files) to function. Each BP file should represent a specific
business logic unit. For example, a database backup task should reside in a single BP file. Encapsulation doesn't
require multiple files but can be achieved through grouping within the designer or IDE.

## Compatibility

Development was primarily conducted on Windows and macOS with testing across:

- **Windows**: Windows 11, Windows Server 2022, Windows Server 2025
- **Linux**: Ubuntu 20.04 / CentOS 8 / Deepin 25
- **macOS**: macOS 15.3

We support both x86_64 and ARM64 platforms. Full testing has been conducted on:

- Macos (M-series ARM chips)
- PC Windows 11 (x86_64)
- PC Linux (x86_64)
- Raspberry Pi 4B (ARM)

If issues arise on other platforms, please report them via GitHub Issues or community discussions.

## Use Cases

### Data Transformation (Intermediate Processing)

Similar to Apache NiFi, data can be ingested, processed, and output using source and destination nodes.

### Automation

Using pre-configured plugins or system handlers, external triggers (e.g., time events, file changes, RESTful APIs,
sockets) can initiate workflows — akin to RPA tools.

### Scheduled Tasks

Supports CRON-based scheduling for recurring tasks.

### Remote Management

Allows real-time control over server environments via scripts and workflows, similar to Jenkins.

## Getting Started

### Prerequisites

1. Rust 1.8+
2. Dual-core CPU with at least 8GB RAM recommended

### Cloning the Repository

```sh
git clone https://github.com/zhaoswen/simx-engine.git
```

### Directory Structure Example

```
filetree 
├── ARCHITECTURE.md
├── LICENSE.txt
├── README.md
├── /account/
├── /bbs/
├── /docs/
│  ├── /rules/
│  │  ├── backend.txt
│  │  └── frontend.txt
├── manage.py
├── /oa/
├── /static/
├── /templates/
├── useless.md
└── /util/
```

### Contributors

Please refer to **CONTRIBUTING.md** for a list of contributors.

#### How to Contribute

Thank you for your interest! Contributions will be accepted starting from version 2025.2 (Summer Edition).

1. Fork the Project
2. Create a Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Version Control

This project uses Git for version management and is hosted on GitHub.

## About

### Authors

Noah Jones

*You can find additional developers in the contributor list.*

### License

The project is licensed under the [MIT License](https://github.com/zhaoswen/simx-engine/blob/master/LICENSE).

### Related Projects

- [Simx Engine](https://github.com/zhaoswen/simx-engine)
- [Simx Engine Extension](https://github.com/zhaoswen/simx-engine-extension)
- [Simx Era Design](https://github.com/zhaoswen/simx-era-design)
- [Rust Programming Language](https://www.rust-lang.org/)