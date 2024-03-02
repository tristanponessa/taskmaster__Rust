
#this file env stdout stderr umask is overriden
echo "start program"
mkdir "./${STARTED_BY}_${RANDOM}${RANDOM}${RANDOM}" #added by env
for i in {1..5}; do
	# Echo current date to stdout
	echo `date`
	# Echo 'error!' to stderr
	echo 'pretend error!' #>&2
	#sleep 1
done
echo "finished program"
