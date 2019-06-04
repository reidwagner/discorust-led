#!/bin/bash

target=$1

if [[ ! $target =~ debug|release ]]; then
    echo "Specify debug or release."
    exit 1
fi

arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabi/$target/discorust-led discorust-led.bin

openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg &
openocd_pid=$!
sleep 1
{
echo reset halt
echo flash write_image erase discorust-led.bin 0x8000000
echo reset
sleep 1
} | telnet localhost 4444
kill $openocd_pid
