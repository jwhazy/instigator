# Instigator

![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![PowerShell](https://img.shields.io/badge/powershell-5391FE?style=for-the-badge&logo=powershell&logoColor=white)
[![Build](https://github.com/jwhazy/instigator/actions/workflows/build.yml/badge.svg)](https://github.com/jwhazy/instigator/actions/workflows/build.yml)

Instigator is a basic, "bring your own libraries" command-line Fortnite launcher.

**If you do not know how to use Command Prompt or PowerShell, please use a different launcher. You will not get support for issues based around the command-line.**

**Instigator does not work on versions later than Chapter 2, Season 6.**

**Versions tested**: 4.1, 5.30, 6.21, 7.30, 7.40, 8.30, 8.51, 10.40, 12.41.

## Features

- **No Windows Defender false positive**
- **GUI-less** Usable in automation, batch scripts and headless/server enviroments.
- **Fully customizable**, use your own libraries and backend.
- **Simple** Instigator only launches the game with no AC, and optionally automatic library injection.
- **Single binary** No dependencies or extraneous libraries required to use Instigator.

## Installation

### WinGet

```
winget install Jacksta.Instigator
```

### Automatic installer

You can download the latest installer [here](https://github.com/jwhazy/instigator/releases/latest/download/Instigator_install.exe).

### Manual install

You can get started by downloading the latest release [here](https://github.com/jwhazy/instigator/releases/download/v1.0.0/instigator.exe). You will need to add Instigator to PATH if you want to use it globally.

## Getting started

1. Prepare and open Instigator folder

```
instigator install
```

3. Add libraries for console, redirect and server. (e.g. console.dll, server.dll, redirect.dll)

4. Add a client to Instigator.

```
instigator add
```

## Useful tools

[Project Reboot](https://github.com/Milxnor/Project-Reboot-3.0) by [Milxnor](https://github.com/Milxnor): stable game server that works with Instigator.

[Cobalt](https://github.com/Milxnor/Cobalt) by [Milxnor](https://github.com/Milxnor): stable SSL bypass that works with Instigator. **Disable automatic console window opening**.
