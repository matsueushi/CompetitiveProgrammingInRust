cd tools
touch score.txt
cat /dev/null > score.txt
cd ..
for i in $(seq -f '%04g' 0 99)
do
    echo ${i}
    cat tools/in/${i}.txt | cargo run --release --bin toyota-hc-2023spring-a > tools/out/${i}.txt
    cd tools
    echo ${i} >> score.txt
    cargo run --release --bin vis in/${i}.txt out/${i}.txt html/${i}.html >> score.txt
    cd ..
done
cd tools
cd ..