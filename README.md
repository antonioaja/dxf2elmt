# dxf2elmt
dxf2elmt is CLI program which can convert .dxf files into .elmt files.

The goal of this program is to create a fast and accurate conversion tool to be used with [QElectroTech](https://qelectrotech.org/). Compared to QET's own conversion [tool](https://github.com/rdsivd/DXFtoQET-2020), dxf2elmt is over 1000x times faster.

## How to Use
dxf2elmt requires only one input from the user, the input file.

For example:

```bash
./dxf2elmt test.dxf
```

The .elmt file be outputted in the same directory as the executable. It will retain the name of the .dxf file.

## Supported Entities

* Lines
* Circles
* Arcs
* Texts
* Ellipses
* Polylines
* LwPolylines
* Solids
* Splines

## To Be Added

* Support for the following
    * Step control for splines
    * Remaining 2d entities

* Better error messages

## Known Issue(s)

* `ParseError(#)`: This occurs due to an improper .dxf file
    * Current fix: Resave .dxf file using a reliable CAD program in order to correct compliant .dxf file

## Compiling

Compiled using Cargo (1.62.0).

## Credits

* [QElectroTech](https://qelectrotech.org/)
* [dxf-rs](https://github.com/IxMilia/dxf-rs)
* [simple-xml-builder](https://github.com/Accelbread/simple-xml-builder)
* [bspline](https://github.com/Twinklebear/bspline)
