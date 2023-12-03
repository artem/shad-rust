set -e
set -x

for dir in $(ls); do
	if [ -f $dir/Makefile ]; then
		make -C $dir
		if [ -f $dir/.subtasks ]; then
			for subtask in $(cat $dir/.subtasks); do
				make -C $dir $subtask
			done
		fi
	fi
done
