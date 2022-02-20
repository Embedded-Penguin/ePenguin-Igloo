# ePenguin-Igloo

## What is Igloo?

Igloo was a full on project and package manager. It is no longer that. Igloo is a tool for creating and maintaining projects. It can create bare metal projects (with no framework), and it can also convert and work with ASF4 (Atmel Start) projects.

Microchip may decide to one day kill atmel start so this project may one day be useless.

## Why does this exist?

I use vim and emacs (on and off) as my text editor. I do not use an IDE.

It aims solves a few problems:
	- Regenerating atmel start projects after recustomizing the board settings via the web configurator should be as simple as one command. It shouldn't be a whole ordeal where if you regenerate your project you have to re-move your source files back into the new generated project.
	- Compiling, executing, and debugging your code should be more streamlined than opening a bunch of terminals that just sit idling.
	- Adding source files to your makefile should be as simple as a command or two. Same with include directories and other configurations.
	- Automate other miscellaneous operations.


## Prerequisites and Assumptions

I'm going to assume you know how to get your desired toolchain. It is a prerequisite.

	- [https://github.com/rizsotto/Bear](bear) (OPTIONAL)
	- [https://github.com/openocd-org/openocd.git](openocd)
	- [https://www.rust-lang.org/tools/install](rust)

These things can also be installed by your package manager if you want to install a binary instead of building from source. I'll update the instructions for different distros at some point.


## Usage

## Installation

```

```
