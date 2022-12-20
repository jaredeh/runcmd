RunCmd
====================

[![CI](https://github.com/jaredeh/runcmd/actions/workflows/ci.yml/badge.svg)](https://github.com/jaredeh/runcmd/actions/workflows/ci.yml)

This library is used for extending `Execute` which is extending `Command` in order to execute commands more easily.  Especially made for simple shell commands returning an exit code as a number, stdout and stderr as strings.

## Usage

```rust
use std::process::Command;

use runcmd::RunCmd;

RunCmd::new("echo \"Hello World\"").execute();

```

### verbose
verbose() will print the ins and outs to stdout

```rust
RunCmd::new("echo \"Hello World\"")
    .verbose()
    .execute();
```

### shell

shell() sets the executor to run the command in a shell using the underlying Execute::shell rather than Execute::command.

```rust
RunCmd::new("echo \"Hello World\"")
    .shell()
    .execute();
```

### executep

executep() runs the command, without returning anything, but panics if the command doesn't succeed.  Useful in only the most trival circumstances.

```rust
RunCmd::new("echo \"Hello World\"")
    .shell()
    .executep();
```

### execute

execute() runs the command, returning a RunCmdOutput.

```rust
let retval: RunCmdOutput = RunCmd::new("echo \"Hello World\"").execute();
```

It returns the following.
```rust
pub struct RunCmdOutput {
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
    pub exitcode: i32
}
```