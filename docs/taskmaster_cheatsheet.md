# INTRO
make light version COPY of project SUPERVISOR http://supervisord.org/

`Supervisor is a client/server system that allows its users to control a number of processes on UNIX-like operating systems`

# SUBJECT
* no root (BONUS can override)
* not a process : does not run in background, you must launch it threw a shell (BONUS can override)
* free lang: RUST
* one config file

# SETUP
```
sudo apt install python3-pip
python3 -m pip3 install supervisor
```

# RULES

* must launch child ps
* must know their status at all times
* config auto loaded at start
* SIGHUP signal reloads config : must not kill unchanged ps but others may be altered
* log system

* light shell
  <br>line edition
  <br>autocompletion
  <br>history
  <br> ...


SHELL CMDS
* status
* start/stop/relaunch ps
* stop all
* reload config without stopping prgm

CONFIG
* which cmd to launch prgm
* nb ps to run and leave runnning
* auto start
* if stopped, restart?
* time elapsed to conclude valid launch
* return code
* nb restart attempts before giveup
* signal to stop prgm gracefully
* time before kill ps if graceful stop tacks too long
* redirect stdout/err to file
* env vars
* dir
* umask

# BONUS
1. root
2. client (supervisorctl): command-line client    
   server (supervisord): DAEMON : job controller
   <br> both communicate threw Unix/TCP sockets
3. advanced logging: email/http/systemLog
4. attached supervised ps to shell and detach to set back to background

# TESTS
* manually kill supervised processes
* launch broken processes
* output a gigantic amount