# Simx Flow Engine

> Eyre Simpson (Noah Jones)
>
> Initial Development: February 13, 2021
>
> Last Update: October 19, 2024
>
> **Licensed under the MIT Open Source License**

## 💡 Introduction

Simx is a lightweight flow engine primarily used for data processing, user operation encapsulation, and automation of
user operations. It does not have a graphical interface and is a command-line (shell) tool. It can be installed as a
system service or registered as the default executor for flow files in the system. Users can use the simx-ui project to
design flows, while this project focuses on the engine component.

Simx serves as a foundational tool for business description. Whether it's Java, C#, JavaScript, Python, Rust, or any
other programming language, they are essentially all describing business processes.

In Simx, the concept of a flow (Flow) is a collection of operations (Handles). These operations can be pre-defined
system operations, shell/powershell/cmd/python scripts, or methods in a jar/dll/so file. In essence, a flow is a
business logic composed of these operations. The design philosophy of Simx is to separate non-essential functionalities
into independent plugins. Users can introduce these plugins into the system as needed, with only the minimal runnable
components retained in the main project, and the rest injected as plugins.

## 🌟 Usage

Simx supports most operating systems and has been tested on Windows 10/11, macOS, and Linux (Ubuntu/CentOS).

You can run the `simx` executable file directly from the `dist` directory. Normally, it will not display anything and
will automatically start the process listener. If you need to expose RESTful APIs, you should add the serve engine
extension.

When the system starts, it will automatically run the flows and scripts located in the `script/init` directory (
configurable via the configuration file). Note that **initialization flows do not have to be cron tasks, but cron tasks
must be initialized and executed**.

The system's scripts and flows are stored in the `flow` and `script` directories, respectively. During the first
startup, the system will actively scan these directories and store the files in the database for API calls. Therefore,
it is recommended to avoid manually adding or deleting these files. The system also provides a manual database refresh
operation, which can be used to synchronize the folder and database before making API calls to prevent data
inconsistencies.

It is recommended to deploy the program on a fast read/write hard drive. For high-concurrency scenarios, mechanical hard
drives are not recommended. If there is no need for persistence, you can enable the in-memory mode in the configuration,
which can significantly improve runtime efficiency.

### Platform Compatibility

Development and testing have been primarily conducted on Windows and macOS, with the following platforms tested:

- Windows 11, Windows Server 2022, Windows Server 2023
- Linux (Ubuntu 20.04, CentOS 7/8)
- macOS 15.1

## Applications

### Data Transformation (Intermediation)

Data can be ingested, processed, and output through specific start nodes (data source nodes) and end nodes (data
terminal nodes), similar to the data transformation capabilities of Nifi.

### Automation

The system can accept external commands or monitor certain signals (such as time, file or folder changes, RESTful,
sockets, etc.) using pre-built functions or plugins, and execute corresponding flows and actions, similar to RPA (
Robotic Process Automation) functionality.

### Scheduled Tasks

The program can perform scheduled tasks based on CRON rules.

### Remote Management

It allows real-time control of the server environment through scripts and flows, similar to some functionalities of
Jenkins.
