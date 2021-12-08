Writes to a file in atomic mode are guaranteed to be atomic up to some size.

This repo provides a rust and python version of tools to attempt to find that limit.

In both cases, each process writes random alphanumeric characters to the file repeatedly. A newline terminates each write to make the file easy to parse. For python, it's possible that multiline writes would behave differently; I know that python has its own buffers but I haven't fully investigated what that means.

The python version loops repeatedly, and if the python version detects a problem it sleeps for 24h.

The rust version runs once.

When verifying a file manually, I've been running `sort /tmp/atomic-appends.txt | uniq -c | awk '{print $1}'`.
