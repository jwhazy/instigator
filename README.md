# Instigator

![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![PowerShell](https://img.shields.io/badge/powershell-5391FE?style=for-the-badge&logo=powershell&logoColor=white)
[![Build](https://github.com/jwhazy/instigator/actions/workflows/build.yml/badge.svg)](https://github.com/jwhazy/instigator/actions/workflows/build.yml)

Instigator is a basic command-line Fortnite launcher I've been working on for the last day and a bit. It is **extremely** basic. It injects console and a redirect/SSL bypass DLL automatically presuming you provide it, if they aren't provided you will need to inject them yourself. I am still learning Rust so if you see anything that could be improved make an issue or if possible, fix it within a pull request. All feedback is greatly appreciated.

**If you do not know how to use Command Prompt or PowerShell, please use a different launcher. You will not get support for issues based around PowerShell and Command Prompt.**

**Versions tested**: 4.1, 5.30, 6.21, 7.30, 7.40, 8.30, 8.51, 10.40, 12.41.

## Features

- **No Windows Defender false positive** 
- **GUI-less** Usable in automation, batch scripts and headless/server enviroments
- **Fully customizable**, use your own DLLs and backend
- **Simple** Instigator only launches the game with no AC, and optionally automatic DLL injection
- **Single binary** No dependencies or extraneous DLLs required to use Instigator.

## Getting started

### WinGet 🆕
If you have WinGet installed (usually installed on latest versions of Windows), you can download Instigator by running `winget install Jacksta.Instigator` in Command Prompt or PowerShell. Once installed, run the program in PowerShell via `instigator install`. This will open Windows Explorer to allow you to drag and drop your console and redirect DLLs in. Make sure they are named `console.dll` and `redirect.dll` respectively. Go back to PowerShell and run `instigator add`. It will provide you with the arguments you need to provide. You can then launch the game using `instigator.exe start {VERSION_NAME}`. You need to run your own backend as Instigator does not provide one **yet**.


### Automatic install
You can download the installer [here](https://github.com/jwhazy/instigator/releases/latest/download/Instigator_install.exe). This will automatically add Instigator to your PATH, you can launch Instigator anywhere on the command-line without changing to the directory it is contained within. Once installed, run the program in PowerShell via `instigator install`. This will open Windows Explorer to allow you to drag and drop your console and redirect DLLs in. Make sure they are named `console.dll` and `redirect.dll` respectively. Go back to PowerShell and run `instigator add`. It will provide you with the arguments you need to provide. You can then launch the game using `instigator.exe start {VERSION_NAME}`. You need to run your own backend as Instigator does not provide one **yet**.


### Manual download
You can get started by downloading the latest release [here](https://github.com/jwhazy/instigator/releases/download/v1.0.0/instigator.exe), or use the installer. Once downloaded, run the program in PowerShell via `.\instigator.exe install`. This will open Windows Explorer to allow you to drag and drop your console and redirect DLLs in. Make sure they are named `console.dll` and `redirect.dll` respectively. Go back to PowerShell and run `.\instigator.exe add`. It will provide you with the arguments you need to provide. You can then launch the game using `.\instigator.exe start {VERSION_NAME}`. You need to run your own backend as Instigator does not provide one **yet**.



