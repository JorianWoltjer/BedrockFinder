# BedrockFinder

Scan the entire World Border in the nether (-3.75m to 3.75m) in 4 minutes. Allows for partial matches with the pattern without performence loss, and is easy to use together with the Fabric Mod to generate code. 

## Usage

For an example of recreating bedrock, see [this post](https://jorianwoltjer.com/blog/post/hacking/part-2-the-new-liveoverflow-minecraft-hacking-server#challenge-2-liveoverflows-base) where I find the coordinates to LiveOverflow's base. 

* You can use the `/getcode` command in [LiveOverflowMod](https://github.com/JorianWoltjer/LiveOverflowMod) to easily get the offsets as Rust code. 
* Then copy the offsets from the chunk start into [`main.rs`](src/main.rs#L44-L55).  
* Put blocks you are 100% sure of in the top `if` statement, and the rest of the blocks in the `let count =` variable. Then make sure to update `MAX_COUNT` to the number of blocks in this `count` variable. 

Finally, you can run the program in its most optimized form with:

```Shell
$ cargo run --release
```

## Example

**Drawing on target screenshot**:

![Drawing on screenshot with bedrock and air](https://jorianwoltjer.com/img/blog/liveoverflow_bedrock_drawing.jpg)

**Recreation in-game**:

![Recreation of bedrock in-game](https://jorianwoltjer.com/img/blog/liveoverflow_bedrock_recreation.jpg)

See the code in [`main.rs`](src/main.rs) for these offsets gotten with `/getcode`.

> **Note**: The hardcoded offsets code in this Github repo are for the following formation as an example:
> ```
>   ██████  
> ████
> ████████
>   ██████
>   ██  ██
> ```

## Timeline

* I created this [`lib.rs`](src/lib.rs) as a proof of concept
* [@user32dll](https://github.com/lcsmnx) and I created a C version ([bedrock_finder](https://github.com/lcsmnx/bedrock_finder)) to brute-force the coordinates orignally
* I remade the brute-force algorithm in Rust (this repository) and cleaned up the code
