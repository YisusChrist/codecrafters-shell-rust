[![progress-banner](https://backend.codecrafters.io/progress/shell/40f0e925-eba2-4b76-b771-736de6880073)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

Table of Contents

- [Introduction](#introduction)
- [Repository Setup](#repository-setup)
- [Passing the first stage](#passing-the-first-stage)
- [Stage 2 \& beyond](#stage-2--beyond)
- [Functionalities implemented for each stage](#functionalities-implemented-for-each-stage)
  - [Basic functionality](#basic-functionality)
    - [Stage 1: Print a prompt](#stage-1-print-a-prompt)
      - [Your Task](#your-task)
      - [Tests](#tests)
      - [Notes](#notes)
    - [Stage 2: Handle missing commands](#stage-2-handle-missing-commands)
      - [Your Task](#your-task-1)
      - [Tests](#tests-1)
      - [Notes](#notes-1)
    - [Stage 3: REPL](#stage-3-repl)
      - [Your Task](#your-task-2)
      - [Tests](#tests-2)
      - [Notes](#notes-2)
    - [Stage 4: The exit builtin](#stage-4-the-exit-builtin)
      - [Your Task](#your-task-3)
      - [Tests](#tests-3)
      - [Notes](#notes-3)
    - [Stage 5: The echo builtin](#stage-5-the-echo-builtin)
      - [Your Task](#your-task-4)
      - [Tests](#tests-4)
    - [Stage 6: The type builtin: builtins](#stage-6-the-type-builtin-builtins)
      - [Your Task](#your-task-5)
      - [Tests](#tests-5)
      - [Notes](#notes-4)
    - [Stage 7: The type builtin: executable files](#stage-7-the-type-builtin-executable-files)
      - [Your Task](#your-task-6)
      - [Tests](#tests-6)
      - [Notes](#notes-5)
    - [Stage 8: Run a program](#stage-8-run-a-program)
      - [Your Task](#your-task-7)
      - [Tests](#tests-7)
      - [Notes](#notes-6)
  - [Navigation](#navigation)
    - [Stage 9: The pwd builtin](#stage-9-the-pwd-builtin)
      - [Your Task](#your-task-8)
      - [Tests](#tests-8)
      - [Notes](#notes-7)
    - [Stage 10: The cd builtin: Absolute paths](#stage-10-the-cd-builtin-absolute-paths)
      - [Your Task](#your-task-9)
      - [Tests](#tests-9)
      - [Notes](#notes-8)
    - [Stage 11: The cd builtin: Relative paths](#stage-11-the-cd-builtin-relative-paths)
      - [Your Task](#your-task-10)
      - [Tests](#tests-10)
      - [Notes](#notes-9)
    - [Stage 12: Home directory](#stage-12-home-directory)
      - [Your Task](#your-task-11)
      - [Tests](#tests-11)
      - [Notes](#notes-10)

# Introduction

Welcome to the Build your own Shell challenge!

A shell is a command-line interface that executes commands and manages processes. In this challenge, you'll build your own POSIX compliant shell that's capable of interpreting shell commands, running external programs and builtin commands like cd, pwd, echo and more.

Along the way, you'll learn about shell command parsing, REPLs, builtin commands, and more.

# Repository Setup

We've prepared a starter repository with some Rust code for you.

Step 1: Clone the repository.

```sh
git clone https://git.codecrafters.io/fec44321de99f891 codecrafters-shell-rust
cd codecrafters-shell-rust
```

Step 2: Push an empty commit.

```sh
git commit --allow-empty -m 'test'
git push origin master
```

When you run the above command, the "Listening for a git push" message below will change, and the first stage will be activated.

# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master
```

Time to move on to the next stage!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.70)` installed locally
2. Run `./your_shell.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
3. Commit your changes and run `git push origin master` to submit your solution
   to CodeCrafters. Test output will be streamed to your terminal.

# Functionalities implemented for each stage

## Basic functionality

Here are the functionalities that you'll need to implement for each stage:

### Stage 1: Print a prompt

#### Your Task

In this stage, you'll implement printing a shell prompt (`$`) and waiting for user input.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

The tester will then check whether your shell prints the `$` prompt and waits for user input.

#### Notes

- There's a space after the `$` character in the prompt.
- Your program must not exit after printing `$`, it should wait for user input.
- We'll handle reading commands and executing them in later stages, this stage only deals with printing the prompt.

### Stage 2: Handle missing commands

#### Your Task

In this stage, you'll implement support for handling missing commands in your shell.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It will then send the following command to your shell:

```sh
$ nonexistent
nonexistent: command not found
```

The tester will check whether your shell prints `<command_name>: command not found` for an unrecognized command.

#### Notes

- The command name will be a random string (like `missing_command_234`), so the response can't be hardcoded.
- We'll handle executing "valid" commands like `echo`, `cd` etc. in later stages, this stage only deals with unrecognized commands.
- In this stage it's okay if your program exits soon after printing the `<command_name>: command not found` message. In later stages we'll check for a REPL (Read-Eval-Print Loop), i.e. whether the shell prints a new prompt after processing each command.

### Stage 3: REPL

#### Your Task

In this stage, you'll implement a [REPL (Read-Eval-Print Loop)](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).

A REPL is an interactive loop that reads user input, evaluates it, prints the result, and then waits for the next input.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It will then send a series of commands to your shell:

```sh
$ invalid_command_1
invalid_command_1: command not found
$ invalid_command_2
invalid_command_2: command not found
$ invalid_command_3
invalid_command_3: command not found
$
```

After each command, the tester will check if `<command_name>: command not found` is printed, and whether a prompt is printed for the next command.

#### Notes

- The exact number of commands sent and the command names will be random.
- Just like the previous stages, all commands will be invalid commands, so the response will always be `<command_name>: command not found`.

### Stage 4: The exit builtin

#### Your Task

In this stage, you'll implement the [exit](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#exit) builtin.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send an invalid command to your shell, followed by the `exit` command:

```sh
$ invalid_command_1
invalid_command_1: command not found
$ exit 0
```

After issuing the `exit 0` command, the tester will verify whether your program terminates with [code/status](https://en.wikipedia.org/wiki/Exit_status) 0.

#### Notes

- The tester will always pass in `0` as the argument to the `exit` command.

### Stage 5: The echo builtin

#### Your Task

In this stage, you'll implement the [echo](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/echo.html) builtin.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a series of `echo` commands to your shell:

```sh
$ echo hello world
hello world
$ echo pineapple strawberry
pineapple strawberry
$
```

After each command, the tester will check if the `echo` command correctly prints the provided text back.

### Stage 6: The type builtin: builtins

#### Your Task

In this stage, you'll implement the `type` builtin command for your shell.

The `type` builtin is used to determine how a command would be interpreted if used. Example:

```sh
$ type echo
echo is a shell builtin
$ type exit
exit is a shell builtin
$ type nonexistent
nonexistent: not found
$ type cat
cat is /bin/cat
```

In this stage we'll only test two cases: builtin commands and unrecognized commands. We'll handle executable files in later stages.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a series of `type` commands to your shell:

```sh
$ type echo
echo is a shell builtin
$ type exit
exit is a shell builtin
$ type type
type is a shell builtin
$ type nonexistent
nonexistent: not found
$
```

The tester will check if the `type` command responds correctly based on the command provided:

- If a command is a shell builtin, the expected output is `<command> is a shell builtin`.
- If a command is not recognized, the expected output is `<command>: not found`.

#### Notes

- The tester will only check for builtin commands and unrecognized commands in this stage.
- `type` itself is a shell builtin command, so `$ type type` should print `type is a shell builtin`.

### Stage 7: The type builtin: executable files

#### Your Task

In this stage, you'll extend the `type` builtin to search for executable files using [PATH](<https://en.wikipedia.org/wiki/PATH_(variable)>).

[PATH](<https://en.wikipedia.org/wiki/PATH_(variable)>) is an environment variable that specifies a set of directories where executable programs are located. When a command is received, the program should search for the command in the directories listed in the PATH environment variable. If the command is found, the program should print the path to the command. If the command is not found, the program should print `<command>: not found`.

#### Tests

The tester will execute your program with a custom `PATH` like this:

```sh
PATH="/usr/bin:/usr/local/bin" ./your_shell.sh
```

It'll then send a series of `type` commands to your shell:

```sh
$ type ls
ls is /usr/bin/ls
$ type abcd
abcd is /usr/local/bin/abcd
$ type missing_cmd
missing_cmd: not found
$
```

The tester will check if the `type` command correctly identifies executable files in the PATH.

#### Notes

- The actual value of the `PATH` environment variable will be random for each test case.
- `PATH` can contain multiple directories separated by colons (`:`), your program should search for programs in each directory in order and return the first match.

### Stage 8: Run a program

#### Your Task

In this stage, you'll add support for running external programs with arguments.

External programs are located using the `PATH` environment variable, as described in previous stages.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a command that you need to execute:

```sh
$ program_1234 alice
Hello alice! The secret code is 1234.
```

The command (`program_1234`) in the example above will be present in `PATH` and will be an executable file.

The tester will check if your shell correctly executes the given command and prints the output.

#### Notes

- The program name, arguments and the expected output will be random for each test case.

## Navigation

### Stage 9: The pwd builtin

#### Your Task

In this stage, you'll implement the `pwd` builtin command.

[pwd](https://en.wikipedia.org/wiki/Pwd) stands for "print working directory".

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a `pwd` command to your shell:

```sh
$ pwd
/path/to/current/directory
$
```

The tester will check if the `pwd` command correctly prints the current working directory.

#### Notes

- The `pwd` command must print the full absolute path of the current working directory.

### Stage 10: The cd builtin: Absolute paths

#### Your Task

In this stage, you'll implement the `cd` builtin command to handle absolute paths.

The `cd` command is used to change the current working directory. `cd` can receive multiple argument types. In this challenge we'll cover:

- Absolute paths, like `/usr/local/bin`. (This stage)
- Relative paths, like `./`, `../`, `./dir`. (Later stages)
- The `~` character, which stands for the user's home directory (Later stages)

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a series of `cd` commands to your shell:

```sh
$ cd /usr/local/bin
$ pwd
/usr/local/bin
$ cd /does_not_exist
cd: /does_not_exist: No such file or directory
$
```

The tester will check if the `cd` command correctly changes the directory when a valid path is provided. It'll also check whether the message `cd: <directory>: No such file or directory` is printed if the provided path is invalid.

#### Notes

- The `cd` command doesn't print anything if the directory is changed successfully. The tester will use `pwd` to verify the current working directory after using `cd`.

### Stage 11: The cd builtin: Relative paths

#### Your Task

In this stage, you'll extend your `cd` builtin command to handle relative paths.

As a recap, `cd` can receive multiple argument types:

- Absolute paths, like `/usr/local/bin`. (Previous stages)
- Relative paths, like `./`, `../`, `./dir`. (This stage)
- The `~` character, which stands for the user's home directory (Later stages)

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a series of `cd` commands to your shell:

```sh
$ cd /usr
$ pwd
/usr
$ cd ./local/bin
$ pwd
/usr/local/bin
$ cd ../../
$ pwd
/usr
$
```

The tester will check if the `cd` command correctly changes the directory when a valid path is provided. It'll also check whether the message `cd: <directory>: No such file or directory` is printed if the provided path is invalid.

#### Notes

- The actual directory names used will be random, so you can't hardcode the expected output.
- Relative paths like `./`, `../`, and more complex relative paths should be handled correctly.
- The `cd` command doesn't print anything if the directory is changed successfully. The tester will use `pwd` to verify the current working directory after using `cd`.

### Stage 12: Home directory

#### Your Task

In this stage, you'll extend your `cd` builtin command to handle the `~` character.

As a recap, `cd` can receive multiple argument types:

- Absolute paths, like `/usr/local/bin`. (Previous stages)
- Relative paths, like `./`, `../`, `./dir`. (Previous stages)
- The `~` character, which stands for the user's home directory (This stage)

The `~` character is shorthand for the user's home directory. When `cd` is received with `~`, your shell should change the current working directory to the user's home directory. The home directory is specified by the [`HOME`](https://unix.stackexchange.com/questions/123858/is-the-home-environment-variable-always-set-on-a-linux-system) environment variable.

#### Tests

The tester will execute your program like this:

```sh
./your_shell.sh
```

It'll then send a series of `cd` commands to your shell:

```sh
$ cd /usr/local/bin
$ pwd
/usr/local/bin
$ cd ~
$ pwd
/home/user
$
```

The tester will check if the `cd` command correctly changes the directory to the user's home directory when `~` is used.

#### Notes

- The `pwd` command will be used to verify the current working directory after using `cd ~`.
- The home directory is specified by the `HOME` environment variable.
