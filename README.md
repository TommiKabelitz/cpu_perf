# CPU perf

A library for interrogating hardware (and software in due course) perfomance metrics. Also a minimal ui through x11 for visualising these metrics.

Lots of work to be done.


### To allow interrogation of all events

This is a temporary workaround

`sudo sysctl -w kernel.perf_event_paranoid=0`