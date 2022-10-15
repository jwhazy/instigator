# Instigator

Instigator is a basic command-line Fortnite launcher I've been working on for the last day and a bit. It is **extremely** basic. It injects console and a redirect/SSL bypass DLL automatically presuming you provide it, if they aren't provided you will need to inject them yourself. I am still learning Rust so if you see anything that could be improved make an issue or if possible, fix it within a pull request. All feedback is greatly appreciated.

**Versions tested**: 4.1, 5.30, 6.21, 7.30, 7.40, 8.30, 8.51, 10.40, 12.41.

## Getting started

**If you do not know how to use Command Prompt or Powershell, please use a different launcher. You will not get support for issues based around PowerShell and Command Prompt.**

You can get started by downloading the latest release [here](https://github.com/jwhazy/instigator/releases/download/v1.0.0/instigator.exe). Once downloaded, run the program in PowerShell via `.\instigator.exe install`. This will open Windows Explorer to allow you to drag and drop your console and redirect DLLs in. Make sure they are named `console.dll` and `redirect.dll` respectively. Go back to Powershell and run `.\instigator.exe add`. It will provide you with the arguments you need to provide. You can then launch the game using `.\instigator.exe start {VERSION_NAME}`. You need to run your own backend as Instigator does not provide one **yet**.

## Features

- No GUI, you can use batch scripts or Windows shortcuts to launch the game
- Usable in automatation and headless/server (no GUI) enviroments
- Fully customizable, use your own DLLs and backend
- Simple, Instigator only launches the game with no AC, and optionally automatic DLL injection
- Single binary
- **~~No Windows Defender false positive~~**
