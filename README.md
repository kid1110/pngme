# pngme
This is my first introductory project in Rust. The project allows adding hidden information to PNG files and can be invoked using the command line.

# How to build
```shell
git clone <project url>
cd pngme/
cargo build --release
```

# How to use
```shell
# get help
./target/release/pngme -h

# encode a message into a png file
# chunk_type would help you find message-data,like hashmap key-value
./target/release/pngme encode ./test.png "chunk_type" "message-data"

#decode and print message-data by message key
./target/release/pngme decode ./test.png "chunk_type"

#remove hidden chunk from png file
 ./target/release/pngme remove ./test.png "chunk_type" 


#print all chunks in png file
./target/release/pngme print ./test.png  
```
# Tips
All chunks in png file including hidden chunks you make would follow png 

