# Rust Command-Line Calculator

A simple command-line calculator written in Rust that supports arithmetic expressions involving decimal and hexadecimal numbers. The calculator allows you to perform addition, subtraction, multiplication, and use parentheses for grouping. You can specify the number base (decimal or hexadecimal) for both input and output.

## Features

- **Number Bases:** Supports decimal (d) and hexadecimal (h) numbers.
- **Arithmetic Operations:** Addition (+), subtraction (-), multiplication (*), and parentheses (( and )).
- **Output Base:** Choose between decimal and hexadecimal output by specifying at the end of the input.
- **Interactive Console:** Provides an interactive prompt for entering expressions with command history support
- **Command Support:**
    - Enter `clear` to clear the console.
    - Enter `q` to quit the program.

## Usage

Input Format

- **Numbers:** Prefix numbers with d for decimal or h for hexadecimal.
    - **Examples:**
        - `d10` represents the decimal number 10.
        - `hA` represents the hexadecimal number A (which is 10 in decimal).

- **Operations:** Use `+`, `-`, `*`, `(`, and `)` as in standard arithmetic expressions.

- **Output Base:** The last character of your input should be either `d` or `h`, indicating the desired output base.

## Examples

**Example 1:** Adding Decimal and Hexadecimal Numbers

```bash
d10 + hA d
```
will output `d20`

**Explanation:** Adds decimal 10 and hexadecimal A (10 in decimal), resulting in 20 displayed in decimal.

**Example 2:** Multiplying Hexadecimal and Decimal Numbers

```bash
hF*d2*hA h
```
will output `h12C`

**Explanation:** Multiplies hexadecimal F (15 in decimal) by decimal 2 by hexadecimal A (10 in decimal), resulting in 15\*2\*10=300, which is displayed as 12C in hexadecimal.

**Example 3** Calculation Precedence (Order of Operations)

```bash
d200 - d10 * d10 d
```
will output `d100`

**Explanation:** Without parentheses, multiplication is performed before subtraction. So it calculates `d200 - (d10 * d10)` which is `200 - 100 = 100` displayed in decimal.

```bash
(d200 - d10) * d10 d
```
will output `d1900`