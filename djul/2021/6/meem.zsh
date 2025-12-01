
for i in {0..9}; do
    for j in {000..999}; do
        # pass="460${i}0${j}00"
        pass="19460${j}00${i}4"
        unzip -P "$pass" -q secrets.zip secret/secret.txt
        if [[ $? -eq 0 ]]; then
            echo $pass
            exit 0
        else
            rm -r secret/ -f
        fi
    done
done

