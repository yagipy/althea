IP = localhost
golang-attack:
	echo "GET http://${IP}" | vegeta attack -duration=10m | tee golang.bin | vegeta report
	rm -f golang.html
	vegeta plot golang.bin > golang.html
	open golang.html

golang-trace:
	curl -o golang.trace "http://${IP}:6060/debug/pprof/trace?seconds=660"
	go tool trace golang.trace

ruby-attack:
	echo "GET http://${IP}" | vegeta attack -duration=10m | tee ruby.bin | vegeta report
	rm -f ruby.html
	vegeta plot ruby.bin > ruby.html
	open ruby.html

java-attack:
	echo "GET http://${IP}" | vegeta attack -duration=10m | tee java.bin | vegeta report
	rm -f java.html
	vegeta plot java.bin > java.html
	open java.html

althea-attack:
	echo "GET http://${IP}" | vegeta attack -duration=5m | tee althea.bin | vegeta report
	rm -f althea.html
	vegeta plot althea.bin > althea.html
	open althea.html
