#!/bin/sh

############
# Usage
# Pass a path to watch, a file filter, and a command to run when those files are updated
#
systemfd --no-pid -s http::8088 -- cargo watch -x run