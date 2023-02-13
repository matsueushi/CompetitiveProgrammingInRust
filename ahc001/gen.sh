for i in $(seq -f '%04g' 0 49)
do
    echo ${i}
    cat tools/in/${i}.txt | cargo run --release --bin ahc001-a > tools/out/${i}.txt &
done
