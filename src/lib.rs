/*!
# RunCmd

This library is used for extending `Execute` which is extending `Command` in order to execute commands more easily.  Especially made for simple shell commands returning an exit code as a number, stdout and stderr as strings.

## Usage

```rust
use std::process::Command;

use runcmd::RunCmd;

RunCmd::new("echo \"Hello World\"").execute();

```

### verbose()
Will print the ins and outs to stdout

```rust
RunCmd::new("echo \"Hello World\"")
    .verbose()
    .execute();
```

### shell()

Sets the executor to run the command in a shell using the underlying Execute::shell rather than Execute::command.

```rust
RunCmd::new("echo \"Hello World\"")
    .shell()
    .execute();
```

### executep()

Runs the command, without returning anything, but panics if the command doesn't succeed.  Useful in only the most trival circumstances.

```rust
RunCmd::new("echo \"Hello World\"")
    .shell()
    .executep();
```

### execute()

Runs the command, returning a RunCmdOutput.

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
*/

extern crate execute;

use std::process::Stdio;

use execute::{Execute, command, shell};

#[derive(Clone)]
pub struct RunCmdOutput {
    pub cmd: String,
    pub stdout: String,
    pub stderr: String,
    pub exitcode: i32
}

pub struct RunCmd {
    retval: RunCmdOutput,
    verbose: bool,
    execute: bool,
    shell: bool
}

impl RunCmd {

    pub fn new(cmd: &str) -> RunCmd {
        RunCmd {
            retval: RunCmdOutput { 
                cmd: String::from(cmd),
                stdout: String::from(""),
                stderr: String::from(""),
                exitcode: 0
              },
            execute: false,
            verbose: false,
            shell: false
        }
    }

    /// Explicitly prints out stdout, stderr, and the exit code for the command run.
    /// But it disables real time output
    #[allow(dead_code)]
    pub fn verbose(&mut self) -> &mut RunCmd {
        self.verbose = true;
        self
    }

    /// Forces the command to run in a system shell.  Can fix some issue with complex commands.
    #[allow(dead_code)]
    pub fn shell(&mut self) -> &mut RunCmd {
        self.shell = true;
        self
    }

    fn print(&self) {
        println!("cmd:\n '{}'\n", self.retval.cmd);
        println!("stdout:\n '{}'\n", self.retval.stdout);
        println!("stderr:\n '{}'\n", self.retval.stderr);
        println!("exitcode: '{}'\n\n", self.retval.exitcode);
    }

    /// Standard execution.  If it doesn't succeed it will just panic.
    pub fn executep(&mut self) {
        self.execute = true;

        let retval = self.execute();

        if retval.exitcode != 0 {
            if self.verbose {
                self.print();
            }
            panic!("Exitcode != 0")
        }
    }

    /// Execution returning a structure with the output: exitcode, stdout, stderr.
    pub fn execute(&mut self) -> RunCmdOutput {
        let mut executor;

        if self.shell {
            executor = shell(&self.retval.cmd)
        } else {
            executor = command(&self.retval.cmd)
        }

        if self.verbose || !self.execute {
            executor.stdout(Stdio::piped());
            executor.stderr(Stdio::piped());
        }

        let output = executor.execute_output().unwrap();

        if let Some(exit_code) = output.status.code() {
            self.retval.exitcode = exit_code;
            self.retval.stdout =  String::from_utf8(output.stdout).unwrap();
            self.retval.stderr =  String::from_utf8(output.stderr).unwrap();
        } else {
            self.retval.exitcode = -1;
            self.retval.stderr =  String::from("Interrupted! in RunCmd");
        }

        if self.verbose {
            self.print();
        }

        return self.retval.clone()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_pass() {
        RunCmd::new("bash -c \"exit 0\"").executep();
    }

    #[test]
    #[should_panic]
    fn execute_fail() {
        RunCmd::new("bash -c \"exit -1\"").executep();
    }

    #[test]
    fn execute_verbose() {
        RunCmd::new("echo bar; exit 0")
            .verbose()
            .execute();
    }

    #[test]
    fn execute_shell() {
        RunCmd::new("echo foobar; exit 0").shell().execute();
    }

    #[test]
    fn execute_output_pass() {
        let retval = RunCmd::new("bash -c \"echo foo; >&2 echo bar; exit -1\"").execute();
        assert_eq!(retval.exitcode, 255);
        assert_eq!(&retval.stdout, "foo\n");
        assert_eq!(&retval.stderr, "bar\n");
        assert_eq!(&retval.cmd, "bash -c \"echo foo; >&2 echo bar; exit -1\"");
    }

    #[test]
    fn execute_output_shell_pass() {
        let retval = RunCmd::new("echo foo; >&2 echo bar; exit -1").shell().execute();
        assert_eq!(retval.exitcode, 255);
        assert_eq!(&retval.stdout, "foo\n");
        assert_eq!(&retval.stderr, "bar\n");
        assert_eq!(&retval.cmd, "echo foo; >&2 echo bar; exit -1");
    }

}
