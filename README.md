# procedural-wallpapers
A collection of wallpaper generators.
Based on a fork of [bagyoni](https://github.com/bagyoni/procedural-wallpapers)s procedural wallpapers project.
Rewritten in Rust.

## How to use (Linux only) TODO
1. Install `libmagick++-dev`, `base-devel` and `cmake`
1. Clone this repository
1. Build the project by typing `cmake . && make TARGET` in the project root with TARGET being one of the algorithms found as .c file in `src/`
1. The compiled program outputs the image given as argument, or `TARGET.png` as default if no argument has been given.

### Command-Line options
```
TARGET - Part of the Procedural Wallpaper Generator
Usage:
  TARGET [OPTION...]

  -s, --random-seed arg  Random Seed (default: <random>)
  -o, --out arg          Output file (default: TARGET.png)
  -H, --help             Print a command-line help text

 Image dimension options:
  -h, --height arg  Image Height (default: 1080)
  -w, --width arg   Image Width (default: 1920)
```

## Generators

<table width="100%">
<tr>
	<td width="50%">
		<img src="examples/clouds.jpg">
		<b>Clouds</b>
		<p>Perlin noise fed into a sigmoid function.
	</td>
	<td width="50%">
		<img src="examples/fern.jpg">
		<b>Fern</b>
		<p>Barnsley fern with mutation.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/flow.jpg">
		<b>Flow</b>
		<p>Perlin flow field.
	</td>
	<td width="50%">
		<img src="examples/islands.jpg">
		<b>Islands</b>
		<p>Perlin noise fed into a cutoff function.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/landscape.jpg">
		<b>Landscape</b> (<a href="https://tyrellrummage.github.io/landscape/">original source</a>)
		<p>Simulated erosion using a midpoint displacement technique.
	</td>
	<td width="50%">
		<img src="examples/lightning.jpg">
		<b>Lightning</b>
		<p>Similar to a Brownian tree but faster to generate.
	</td>
	
</tr>
<tr>
	<td width="50%">
		<img src="examples/marrowlike.jpg">
		<b>Marrowlike</b> (<a href="http://pcg.wikidot.com/forum/t-79282/multiplicative-cascades-ish">original source</a>)
		<p>If a pixel is too dark, give it a random brightness. Scale to double size. Repeat.
	</td>
	<td width="50%">
		<img src="examples/mesh.jpg">
		<b>Mesh</b>
		<p>Transformations of the complex plane.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/tangles.jpg">
		<b>Tangles</b>
		<p>Rec-tangles.
	</td>
	<td width="50%">
		<img src="examples/water.jpg">
		<b>Water</b>
		<p>Very simple interference pattern generator.
	</td>
</tr>
<tr>
	<td width="50%">
		<img src="examples/wood.jpg">
		<b>Wood</b>
		<p>Modular Perlin noise stretched along the x axis.
	</td>
	<td width="50%">
		<img src="examples/zebra.jpg">
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