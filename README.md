# discord-ipc-bridge

Learning rust with websockets and named pipes.

Exposes a websocket server that talks to the discord named pipes (domain sockets on unix).

Why?

 1) Discord RPC is locked down and lame
 2) Web extensions can only talk websocket/http


## Windows Jank Install

```ini
[discord-ipc-bridge]
startup="C:\path\to\discord-ipc-bridge.exe"
shutdown_method=winmessage
```
```bat
srvstart.exe install discord-ipc-bridge -c path/to/inifile.ini
```
