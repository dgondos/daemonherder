# daemonherder

## Overview

A basic service manager written in Rust. At this stage, it's a toy project for me to play around a bit with Rust.

## Dependencies

* Rust 1.0.0

* rustc-serialize: Edit `Cargo.toml` and replace the local dependency `dependencies.rustc-serialize` with a general dependency section from crates.io (which pulls rustc-serialize)

* Tested on Linux only for now, but should work on OSX.

## Configuration

### .conf naming convention

Each service you want daemonherder to manage needs its own `.conf` file in `$HOME/.dherder`.

You can either create a conf file for your service as `myservice.conf`, in which case daemonherder will run that service unconditionally.

Otherwise, you can name your conf file in this format: `myservice@host.conf`, in which daemonherder will only run this service if it's running on `host`. This enables you to keep your conf files in one place, for example in git, deploy them on all machines and they'll run automatically on the right one.

### .conf structure

Each .conf file consists of a JSON structure, with the following layout:

```
{
    "name": "Descriptive name of service",
    "cwd": "/working/path/of/service",
    "cmd": "command_to_run",
    "args": ["--argument_to_your_service",
             "--argument_with_value",
             "2",
             "-v"]
}
```

### Keeping daemonherder running

Once you set up all the conf files, the best way to go is to create an entry in your crontab to keep daemonherder running across reboots: `@reboot /path/to/daemonherder`. daemonherder itself normally never exits. All subprocesses are handled in separate threads and each thread ensures that their child process restarts automatically if it ever quits or crashes, forever.

## TODOs

A lot :)

* Detecting config file changes without having to restart all services
* Proper stdout/stderr handling of child processes
* Alerting/monitoring interfaces
* This project should evolve alongside Rust itself. There's a bunch of things which I would've done differently if they were stable in Rust. As Rust's stdlib matures, there should be some rewrites here and there.