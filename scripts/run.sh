#!/bin/bash

cargo build
arm-none-eabi-gdb -q -ex "target extended-remote :3333" -ex "monitor arm semihosting enable" target/thumbv6m-none-eabi/debug/rp2040-project-template
