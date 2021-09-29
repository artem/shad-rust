set -e
set -x

for dir in $(ls); do
	if [ -f $dir/Makefile ]; then
		make -C $dir
	fi
done