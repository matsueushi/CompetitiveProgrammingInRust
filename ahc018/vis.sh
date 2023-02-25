cd tools
touch score.txt
cat /dev/null > score.txt
cd ..
for i in $(seq -f '%04g' 0 99)
do
    echo ${i}
    cat tools/in/${i}.txt | ../target/release/tester cargo run --release --bin ahc018 > tools/out/${i}.txt
    cd tools
    cargo run --release --bin vis in/${i}.txt out/${i}.txt png/${i}.png >> score.txt
    cd ..
done
cd tools
awk '{n += $1}; END{print n}' score.txt
cd ..