## Simx Quantum Engine

**Introduction:**
Simx Engine is a compact execution engine primarily utilized in data processing, user action encapsulation, and user
action automation. It operates as a command-line (shell) tool and can be installed as a system service through the
provided installation tool or registered as the default executer of blueprint files in the system.

**User Interface:**
Simx Engine does not have an interface. Instead, it provides a command-line interface for user interaction.

**Installation and Configuration:**
Users can install Simx Engine as a system service using the provided installation tool. Alternatively, they can register
it as the default executer of blueprint files in the system.

**Design and Development:**
Simx Engine’s design philosophy revolves around modularity and extensibility. It aims to separate non-essential
functions into independent plug-ins, allowing users to introduce and integrate these plug-ins into their systems as
needed. The main project retains only the essential components, while the remaining functionalities are injected as
plug-ins.

**Performance Optimization:**
Simx Engine prioritizes performance while simultaneously mitigating the complexities associated with intricate logic.
This approach addresses potential challenges in the future.

**Basic Concepts:**
**Processor (Handler):** A processor is an executable operation that can be a system, a shell/powershell/CMD/python
script, or a method within a jar/DLL/so.
**Blueprint:** A blueprint is a collection of business logic and a specific file. Upon instantiation by the engine, one
or more flows can be formed to execute specific operations.
**Flow:** A flow is a business logic composed of processors. It can contain multiple processors and one or more groups.
A group can contain multiple processors, and the execution order of processors within a group is defined by the user.
The execution order of processors within a group can also contain nested groups.
**Components:** A component is a group of processors that can contain multiple processors. The order of execution of
processors within a group is defined by the user, and the order of execution of processors within a group can contain
nested groups.
**Services (Commands):** A service can be simply viewed as a continuously running processor that listens for triggering
streams or scripts (e.g., CRON service) or continuously provides some functionality (e.g., MySQL service).
**Extensions:** Extensions are a collection of processors and services that currently enable users to provide extensions
through the connection library. They will gradually support Melt Core access extensions.

## Usage

The Simx engine offers various forms, depending on specific needs, can be packaged as a dynamic link library (dll, so,
dylib, corresponding to the library package in the project) or executable file (corresponding to the runner package in
the project). If provided as a dynamic link library, users can use any language that supports the FFI standard.
A language that supports the FFI standard can be called or run as an executable file, which starts a thread listening to
receive instructions.

The engine requires a standard Simx Blueprint file (blueprint file) to function properly. Each BP file should represent
a specific business, such as the backup service used to automatically back up the database should exist in a separate BP
file. In Simx, encapsulation does not require multiple layers.
For example, the user can combine one or more processors (also known as grouping, packaging), and the combined processor
group (also known as a component) can be used like any other normal processor.

## Compatibility

The development content is primarily conducted on Windows and macOS, and the platforms tested include, but are not
limited to:

- Windows 11, Windows Server 2022, Windows Server 2025
- Linux (Ubuntu 20.04 / Centos 8 / Deepin 25)
- macOS 15.3

We support both X86_64 and ARM64 platforms, although our current availability is limited to macOS M series chips (ARM),
PC Windows 11 (X86_64), PC Linux (X86_64), and Raspberry Pi 4B (X86_64).

Our application is fully tested on ARM platforms.

Should any issues arise during runtime, we welcome feedback via the issue tracker or community forums.

## Application Scenario

### Data Conversion (Transit)

Data can flow in, out, and out through designated start nodes (data source nodes) and end nodes (data destination
nodes), similar to Nifi’s data transformation capabilities.

### Automated Operation

You can accept external instructions or listen to specific signals (such as time, file or folder changes, RESTful,
socket, etc.) through the system’s predefined functions or plugins, and execute corresponding processes and actions,
akin to the functionality of RPA.

### Timed Tasks

Programs can implement scheduled tasks based on CRON rules.

### Remote Management

Allows real-time control of the server environment via scripts and processes, similar to some of the functionality of
Jenkins.