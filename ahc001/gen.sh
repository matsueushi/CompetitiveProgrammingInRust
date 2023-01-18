for i in $(seq -f '%04g' 1 49)
do
    echo ${i}
    cat tools/in/${i}.txt | cargo run --bin ahc001-a > tools/out/${i}.txt &
done
