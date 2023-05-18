# roblox-channel-bruteforcer

the better funny setup.rbxcdn.com wordlist thing:tm:
## Install

1. Clone the repository via `git`:

    ```txt
    git clone https://github.com/Roblox-Devs/roblox-channel-bruteforcer.git
    cd roblox-channel-bruteforcer
    ```
2. Build via [cargo](https://doc.rust-lang.org/cargo/getting-started/index.html):

    ```txt
    cargo build
    ```
## Usage
``roblox-channel-bruteforcer.exe --help``
```txt
rbx channel bruteforcer 1.1

USAGE:
    roblox-channel-bruteforcer.exe [OPTIONS] --output <output> --prefix <prefix> --wordlist <wordlist>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --wordlist <wordlist>    wordlist to use
        --prefix <prefix>        prefix for channel name (ex: zproject)
        --output <output>        name of output file
        --threads <threads>      number of worker threads you want used [default: 250]
```

### Extra Notes

* To quit the process and stop the session, you need to use a keyboard interrupt (`Ctrl+C`) in your terminal, or just cancel the process directly through whatever you're using.

* If you get immediate request errors, try toggling `--threads` to a number lower than the default of `250`.