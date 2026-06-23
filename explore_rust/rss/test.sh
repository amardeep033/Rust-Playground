# One liner - prints peak RSS when process ends
PID=416585; \
PEAK=0; \
while kill -0 $PID 2>/dev/null; do \
    RSS=$(grep VmRSS /proc/$PID/status 2>/dev/null | awk '{print $2}'); \
    [ ! -z "$RSS" ] && [ "$RSS" -gt "$PEAK" ] && PEAK=$RSS; \
    sleep 1; \
done; \
echo "Peak RSS: $(echo "scale=2; $PEAK/1024" | bc) MB"