# `tupperplot` 📐

**Tupper's self-referential formula** plotting itself on a framebuffer... and Pacman and the Euler's identity too!

## What's this formula?

<br>
<div align="center">
  <table>
    <tr>
      <td style="padding: 10px;"><img src="https://wikimedia.org/api/rest_v1/media/math/render/svg/2c74e5046994470b148f2e386aebd1f12baa88a8"></td>
    </tr>
  </table>
</div>
<br>

Tupper's self-referential formula is a formula that visually represents itself when graphed at a specific location in the (x, y) plane.<sup>[<a href="https://en.wikipedia.org/wiki/Tupper%27s_self-referential_formula">src</a>]</sup>

### Resources

- [**Wikipedia**](https://en.wikipedia.org/wiki/Tupper%27s_self-referential_formula)
- [**Numberphile's Video**](https://www.youtube.com/watch?v=_s5RFgd59ao)

## Installation

**Compile:**

```
$ git clone https://github.com/mufeedvh/tupperplot.git
$ cd tupperplot/
$ cargo run --release
```


**Linux Dependencies:**

```shell
$ sudo apt install libxkbcommon-dev libwayland-cursor0 libwayland-dev
```

## Usage

**Generate Tupper's self-referential formula itself:**

```
$ tupperplot tuppers
```

**Generate Pacman Plot:**

```
$ tupperplot pacman
```


**Generate Euler's Identity Plot:**

```
$ tupperplot euler
```

### Implementation Details

- `BigNumber` / `BigInt` - https://github.com/rust-num/num-bigint
- Bitmap Rendering - https://github.com/sondrele/rust-bmp
- Bitmap Manipulation - https://github.com/AlecDivito/rustbitmap
- Framebuffer - https://github.com/emoon/rust_minifb

---
