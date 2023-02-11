cd tools
for i in $(seq -f '%04g' 0 49)
do
    cargo run --release --bin vis in/${i}.txt out/${i}.txt svg/${i}.svg
done
cd ..