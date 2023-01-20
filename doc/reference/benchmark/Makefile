IP = localhost

golang-attack:
	echo "GET http://${IP}" | vegeta attack -duration=5m | tee golang.bin | vegeta report
	rm -f golang.html
	vegeta plot golang.bin > golang.html
	open golang.html

golang-trace:
	curl -o golang.trace "http://${ARG}:6060/debug/pprof/trace?seconds=360"
	go tool trace golang.trace
