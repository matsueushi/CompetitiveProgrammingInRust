cd tools
touch score.txt
cat /dev/null > score.txt
for i in $(seq -f '%04g' 0 10)
do
    cargo run --release --bin vis in/${i}.txt out/${i}.txt png/${i}.png >> score.txt
done
awk '{n += $1}; END{print n}' score.txt
cd ..