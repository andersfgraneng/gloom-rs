---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 2
author:
- Anders Finserås Graneng
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
#- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Excercises 

## Task 1 

### b)

![
    The resulting triangle of task 1b
](images/a2_1b.png)

What happens here is that opengl blends the colors between the defined vertices.

## Task 2

### a)

![
    The resulting triangles of task 2a
](images/a2_2a.png)

### b)

#### i)

When swapping the colors of the different triangles we could see that the color of the portions where they overlapped changed. By swapping the colors this occured because the triangles are rendered at different depths with different opacity. The opacity and order of the triangles in an overlapping area matters because the intensity of the colors will contribute differently due to this.

#### ii)

When swapping the depth of the triangles we witness the same effect, but due to changing the depth of the triangle themselves instead of the colors with different opacity.

## Task 3

### b)

| Variable   | Result    |
|--------------- | --------------- |
| a   | Modifying this variable results in shearing on the x-axis   |
| b   | Modifying this variable results in shearing on the z-axis along the x-axis   |
| c   | Modifying this variable results in translation on the x-axis |
| d   | Modifying this variable results in shearing on the y-axis   |
| e   | Modifying this variable results in shearing on the z-axis along the y-axis    |
| f   | Modifying this variable results in translation on the y-axis   |

### c)

None of the transformations observed were rotations because the distance of all the vertices were not the same, but always changed.

## Task 4


