#!/bin/bash

SLEEP_INTERVAL=${SLEEP_INTERVAL:-60}

while true; do
    /app/bin/take-snapshot
    
    sleep $SLEEP_INTERVAL
done
