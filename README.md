# Lightswitch
> Allow anyone online to flip on a process on your machine! 💡

Lightswitch is a simple program that hosts a website (using [btnify](https://github.com/NotThatRqd/btnify))
that allows anyone online to start a process on your PC. Please
only use Lightswitch if you know what you're doing, because allowing
anyone on the internet to start a process on your PC can be potentially
dangerous.

Lightswitch was originally made to allow my friends and I to turn on
our Minecraft server without having to message the person hosting it,
however Lightswitch can be used for anything you want.

# How to use

To use Lightswitch simply create a file named `lightswitch_config.toml`
wherever you are running the lightswitch executable.

Minecraft server

```toml
# CHANGE THIS to an address accessible online to your friends
addr = "0.0.0.0:3000"
password = "very secret"

[process_info]
cwd = "C:\\Users\\rad\\Documents\\testserver"
cmd = "java"
args = ["-Xmx1g", "-jar", "paper-1.20.1-196.jar", "--nogui"]
```

Other

```toml
addr = "0.0.0.0:3000"

[process_info]
cwd = './'
cmd = 'dummy.exe'
args = []
```

Image of what the website looks like:

![image](https://github.com/NotThatRqd/lightswitch/assets/67846317/ab734927-1aae-42f0-8063-327678548b92)
