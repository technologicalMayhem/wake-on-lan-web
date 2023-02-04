# Wake on Lan - Web

Used to wake up or shutdown a machine on the network via a HTTP request from outside using the ``/wake`` and ``/shutdown`` endpoints.
I made this expecting a guacamole server to run on the machine so it will show a ready state on the ``/state`` endpoint if the guacamole server is available.

## Running it

To run set the following environment variables:

```
WAKE_PORT=8080                  # The port the app will accept http request on
MACHINE_MAC=ff:ff:ff:ff:ff:ff   # The mac address of the machine to wake up
MACHINE_IP=192.168.178.10       # The ip address of the machine to wake up
USERNAME=johndoe                # The username of the user on the machine
GUACAMOLE_PORT=8080             # The port guacamole runs on
```

I recommend placing the server behind a reverse proxy.