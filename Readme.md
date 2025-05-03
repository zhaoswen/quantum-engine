<!--suppress CheckImageSize -->
<br />

<p align="center">
  <a href="https://github.com/zhaoswen/simx-engine/">
    <img src="runner/resource/SimxIcon.png" alt="Logo" width="80px" height="80px">
  </a>

<h3 align="center">Simx Quantum Engine</h3>
  <p align="center">
    适用于业务人员编码的解决方案
    <br />
    <a href="https://pro.zhaosw.site/docs/engine/quick-start"><strong>探索本项目的文档 »</strong></a>
    <br />
    <br />
    <a href="https://github.com/zhaoswen/simx-engine">设计器</a>
     · 
    <a href="https://github.com/zhaoswen/simx-engine/issues">Bug</a>
     · 
    <a href="https://github.com/zhaoswen/simx-engine/issues">需求</a>
  </p>

## 文档目录

- [介绍](#介绍)
- [设计](#设计)
- [概念](#概念)
- [使用](#使用)
- [兼容性](#兼容性)
- [应用场景](#应用场景)
- [上手指南](#上手指南)
  - [开发前的配置要求](#开发前的配置要求)
  - [克隆代码](#克隆代码)
- [文件目录说明](#文件目录说明)
- [贡献者](#贡献者)
  - [如何参与开源项目](#如何参与开源项目)
- [版本控制](#版本控制)
- [作者](#作者)
- [关于](#关于)

## 介绍

Simx Engine 是一个精简的执行引擎，主要应用于数据处理、用户操作封装、用户操作自动化等场景。它本身没有界面，是一个命令行（shell）工具，
可以通过安装工具将其作为一个系统服务进行安装， 或注册为系统中蓝图文件的默认执行器；

用户通过 [Simx Era Design](https://github.com/zhaoswen/simx-era-design) 或 Melt Studio 实现流设计的工作，本项目为引擎部分。

## 设计

simx的设计思路是所有非必须的功能尽可能拆分出去，作为独立的插件，用户需要使用可以将其引入到系统中，主项目中仅保留最小可运行部分，其余均作为插件形式注入；
引擎关注性能，同时也尽可能屏蔽复杂逻辑引起的使用困难，这也是未来可能的挑战之一。

## 概念

- **处理器（Handler）**：处理器是一个可执行的操作，可以是系统自带的，也可以是shell/powershell/cmd/python脚本，也可以是一个jar/dll/so中的方法，
- **蓝图（Blueprint）**：蓝图是一个业务逻辑的集合，是一个具体的文件，一个蓝图经过引擎进行实例化后，即可形成一个或多个流（Flow）执行具体的操作
- **流（Flow）**：流是由处理器组成的业务逻辑，可以包含多个处理器，也可以包含一个或多个组，组内可以包含多个处理器，组内处理器的执行顺序由用户定义，组内处理器的执行顺序可以包含嵌套组
- **组件（Component）**：组件是一个组，组内可以包含多个处理器，组内处理器的执行顺序由用户定义，组内处理器的执行顺序可以包含嵌套组
- **服务（Command）**：服务可以简单看作持续运行的处理器，可以监听触发流或脚本（CRON服务），或持续的提供一些功能（MySQL服务）
- **扩展（Extension）**：扩展是一系列处理器和服务的集合，目前支持用户通过连接库提供扩展功能，后续会逐渐支持 Melt Core 接入扩展

## 使用

Simx 引擎具有多种形态，根据具体需要可以打包为动态链接库（dll、so、dylib，对应项目中的library包）或者可执行文件（对应项目中的runner包），如果作为一个动态连接库的形式提供，用户可以通过任意
支持ffi标准的语言进行调用，也可以作为可执行文件运行，这种方式会启动一个线程监听，以接受指令。

引擎需要标准的Simx Blueprint文件（蓝图文件）才能正常运行，每个BP文件都应代表一个具体的业务，比如用于自动备份数据库的backup业务应该存在于一个单独的BP文件中，在simx中，封装并不需要通过多个
文件进行，而是直接通过设计器或IDE的组概念实现，比如，用户可以将一个或多个处理器进行组合操作（也可称为打组、封装），这种组合后的处理器组（也称为组件）可以和其他正常处理器一样进行使用

## 兼容性

开发内容主要在Windows和Macos上进行，测试的平台包括但不限于：

- Windows 11、Windows Server 2022、Windows Server 2025
- Linux（Ubuntu 20.04 / Centos 8 / Deepin 25）
- Macos 15.3

我们同时支持X86_64和ARM64平台，但目前仅在 Macos M系列芯片（ARM）、PC Windows 11（X86_64）、PC Linux（X86_64）、Raspberry Pi 4B (
ARM)上进行过完整测试，
如果运行时发现问题，欢迎通过issue或社区进行反馈。

## 应用场景

### 数据转换（中转）

可以通过特定的开始节点（数据源节点）和结束节点（数据终端节点），实现数据的流入、操作和流出，类似于Nifi的数据转换功能

### 自动化操作

可以通过系统预置的功能或插件接受外部的指令或监听某些信号（如时间、文件或文件夹变动、restful、socket等），并执行相应的流程和动作，类似于RPA的功能

### 定时任务

程序可以根据CRON规则，实现定时任务

### 远程管理

允许通过脚本和流程，实现服务器上环境的实时控制，类似于Jenkins的部分功能

## 上手指南

###### 开发前的配置要求

1. Rust 1.8+
2. 建议 双核心 CPU + 8G 内存及以上

###### **克隆代码**

```sh
git clone https://github.com/zhaoswen/simx-engine.git
```

### 文件目录说明

eg:

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

### 贡献者

请阅读**CONTRIBUTING.md** 查阅为该项目做出贡献的开发者。

#### 如何参与开源项目

感谢一切的贡献者，目前暂不接受贡献，在版本迭代到 2025.2 夏季版后，正式开始接受贡献。

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### 版本控制

该项目使用Git进行版本管理，通过 Github 进行托管。

## 关于

### 作者

Noah Jones

*您也可以在贡献者名单中参看所有参与该项目的开发者。*

### 版权说明

项目采用 [MIT LICENSE](https://github.com/zhaoswen/simx-engine/blob/master/LICENSE) 授权许可

### 相关内容

- [Simx Engine](https://github.com/zhaoswen/simx-engine)
- [Simx Engine Extension](https://github.com/zhaoswen/simx-engine-extension)
- [Simx Era Design](https://github.com/zhaoswen/simx-era-design)
- [Rust Programming Language](https://www.rust-lang.org/)