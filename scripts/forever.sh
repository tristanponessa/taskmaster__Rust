#!/bin/bash
while true
do
	# Echo current date to stdout
	echo `date`
	echo $STARTED_BY
	echo "$PWD"
	# Echo 'error!' to stderr
	echo 'pretend error!' >&2
	sleep 1
done