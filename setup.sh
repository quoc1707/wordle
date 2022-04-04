while getopts "c:" opt
do
    case "$opt" in
        c) command=${OPTARG};;
    esac
done
if [ $command == 'build' ]
then
    cargo build --release
elif [ $command == 'serve' ]
then
    cargo run --release
elif [ $command == 'clean' ]
then
    cargo clean
else
    cargo install --path .
fi