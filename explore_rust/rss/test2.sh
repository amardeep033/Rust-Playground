#!/usr/bin/env bash

PROC_NAME="rss"
OUT_FILE="rss_peak.log"
INTERVAL=0.3

echo "Starting RSS tracker..." > "$OUT_FILE"
echo "----------------------------------" >> "$OUT_FILE"

declare -A PEAK

# track MIN available RAM
MIN_AVAIL_KB=999999999999

while true; do
    PIDS=$(pgrep -f "$PROC_NAME")

    # read available memory
    CUR_AVAIL_KB=$(grep MemAvailable /proc/meminfo | awk '{print $2}')
    if [ "$CUR_AVAIL_KB" -lt "$MIN_AVAIL_KB" ]; then
        MIN_AVAIL_KB=$CUR_AVAIL_KB
    fi

    [ -z "$PIDS" ] && break

    for PID in $PIDS; do
        kill -0 "$PID" 2>/dev/null || continue

        CMD=$(tr '\0' ' ' < /proc/$PID/cmdline 2>/dev/null)
        ARGS=$(echo "$CMD" | sed -E "s/.*$PROC_NAME[ ]*//")
        KEY="$ARGS"

        RSS=$(grep VmRSS /proc/$PID/status 2>/dev/null | awk '{print $2}')
        [ -z "$RSS" ] && continue

        CUR=${PEAK[$KEY]:-0}

        if [ "$RSS" -gt "$CUR" ]; then
            PEAK[$KEY]=$RSS
        fi
    done

    sleep "$INTERVAL"
done

echo "" >> "$OUT_FILE"
echo "Peak RSS Report" >> "$OUT_FILE"

for KEY in "${!PEAK[@]}"; do
    KB=${PEAK[$KEY]}
    if [ "$KB" -lt 5000 ]; then
        continue
    fi

    MB=$(echo "scale=2; $KB/1024" | bc)
    echo "$PROC_NAME $KEY : $MB MB" >> "$OUT_FILE"
done

MIN_AVAIL_MB=$(echo "scale=2; $MIN_AVAIL_KB/1024" | bc)
echo "during run min available space: ${MIN_AVAIL_MB} MB" >> "$OUT_FILE"

echo "RSS tracking completed." >> "$OUT_FILE"
