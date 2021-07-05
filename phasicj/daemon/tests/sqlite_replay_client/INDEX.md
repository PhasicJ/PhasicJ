# PhasicJ Daemon Test Replay Client

A gRPC client for use in testing. It connects to the PhasicJ daemon over a
UNIX domain socket (UDS), reads from a pre-recorded SQLite database file of
PhasicJ events, and sends all of these events to the daemon over this socket.
