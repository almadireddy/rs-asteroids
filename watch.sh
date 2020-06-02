#!/bin/sh

############
# Usage
# Pass a path to watch, a file filter, and a command to run when those files are updated
#
# Example:
# watch.sh "node_modules/everest-*/src/templates" "*.handlebars" "ynpm compile-templates"
############

watch() {
    WORKING_PATH=$(pwd)
    DIR="./"
    FILTER="*.rs"
    COMMAND="wasm-pack build ./app --target web --out-dir www/wasm"
    chsum1=""

    browser-sync start --server "./app/www" --files "./app/www/**/*" & BS_PID=$!

    trap "kill $BS_PID; exit" SIGHUP SIGINT SIGTERM
    
    while [[ true ]]
    do
        chsum2=$(find -L $WORKING_PATH/$DIR -type f -name "$FILTER" -exec md5 {} \;)
        if [[ $chsum1 != $chsum2 ]] ; then
            echo "Found a file change, executing $COMMAND..."
            $COMMAND
            chsum1=$chsum2
        fi
        sleep 2
    done 
}

watch "$@"
