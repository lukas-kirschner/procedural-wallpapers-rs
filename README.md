# procedural-wallpapers in Rust
A collection of wallpaper generators.
Based on a fork of [bagyoni](https://github.com/bagyoni/procedural-wallpapers)s procedural wallpapers project in C++ ([link](https://github.com/lukas-kirschner/procedural-wallpapers)).
Rewritten in Rust.

## How to use
1. Set up your Rust toolchain with Cargo
2. Clone this repository
3. Build the project by typing `cargo build --release` 
4. To get command-line help, run the build binary with the `--help` flag.

### Command-Line options
```text
procedural_wallpapers 0.1.0
Generate wallpapers procedurally with the given algorithm

USAGE:
    procedural_wallpapers [OPTIONS] --mode <MODE> --output <OUTPUT>

OPTIONS:
    -h, --height <HEIGHT>    Desired height (pixels) of the generated image [default: 1080]
        --help               Print help information
    -m, --mode <MODE>        Image generation mode [possible values: clouds]
    -o, --output <OUTPUT>    The output file to save
    -s, --seed <SEED>        Seed for the random number generator. If a seed of 0 is given, no seed
                             is used [default: 0]
    -V, --version            Print version information
    -w, --width <WIDTH>      Desired width (pixels) of the generated image [default: 1920]

```

## Generators and examples

All the example images shown below were generated with a size of `400`x`400` pixels and a seed of `1234`.

<table width="100%">
<tr>
	<td width="50%">
		<img src="examples/clouds.png">
		<b>Clouds</b>
		<p>Perlin noise fed into a sigmoid function.
	</td>
	<td width="50%">
		<img src="examples/flow.png">
		<b>Flow</b>
		<p>Perlin flow field.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/islands.png">
		<b>Islands</b>
		<p>Perlin noise fed into a cutoff function.
	</td>
    <td width="50%">
		<p>REMOVED due to licensing issues</p>
		<b>Fern</b>
		<p>Barnsley fern with mutation.
	</td>
</tr>
<tr>
	<td width="50%">
				<p>REMOVED due to licensing issues</p>
		<b>Landscape</b> (<a href="https://tyrellrummage.github.io/landscape/">original source</a>)
		<p>Simulated erosion using a midpoint displacement technique.
	</td>
	<td width="50%">
		<img src="examples/lightning.png">
		<b>Lightning</b>
		<p>Similar to a Brownian tree but faster to generate.
	</td>
	
</tr>
<tr>
	<td width="50%">
				<p>REMOVED due to licensing issues</p>
		<b>Marrowlike</b> (<a href="http://pcg.wikidot.com/forum/t-79282/multiplicative-cascades-ish">original source</a>)
		<p>If a pixel is too dark, give it a random brightness. Scale to double size. Repeat.
	</td>
	<td width="50%">
		<img src="examples/mesh.png">
		<b>Mesh</b>
		<p>Transformations of the complex plane.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/tangles.png">
		<b>Tangles</b>
		<p>Rec-tangles.
	</td>
	<td width="50%">
		<img src="examples/water.png">
		<b>Water</b>
		<p>Very simple interference pattern generator.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/wood.png">
		<b>Wood</b>
		<p>Modular Perlin noise stretched along the x axis.
	</td>
	<td width="50%">
		<img src="examples/zebra.png">
		<b>Zebra</b>
		<p>The same complex function as in the Mesh program, but from a different perspective.
	</td>
</tr>
</table>

## Disclaimers

##### From the forked repository:
* The *landscape* generator is shamelessly stolen from here: https://tyrellrummage.github.io/landscape/
* The *marrowlike* generator follows the recursive algorithm described here: http://pcg.wikidot.com/forum/t-79282/multiplicative-cascades-ish
* The *fern* generator uses a fractal image compression method that might be patented (I couldn't find much information about it)

## Licensing
All programs, except for those listed in the Disclaimers section or in their respective source file, are in the public domain. Feel free to use them as you like.