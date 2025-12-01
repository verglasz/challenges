
normal=(4 6 4 8 3 5 6 8)

for i in {1..8}; do
    chars=$(head -n $i pass | tail -n1 | wc -c)
    rest=normal[$i]
    echo $(( ($chars + 2*$rest )/3 ))
done

