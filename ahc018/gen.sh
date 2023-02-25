for i in $(seq -f '%04g' 0 99)
do
    echo ${i}
    cat tools/in/${i}.txt | ../target/release/tester cargo run --release --bin ahc018 > tools/out/${i}.txt
done