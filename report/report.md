---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Exercise 1
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

### c)

![
    The resulting five triangles of task 1c
](images/1c.png)

## Task 2

### a)

![
    The resulting triangle of task 2a
](images/2a.png)

#### i)
The name of this phonomenon is clipping. Thats why we see the edges of the triangle is gone.

#### ii)
This happens when we try to render elemen outside the bounds of the clip box. As stated in the opengl guide for this exercise the clip box is a 2x2x2 cube. This means the bounds for all axises are from -1.0 to 1.0, therefore the coordinates for the z-axis are out of bounds. Since the z-axis is also the axis in which the viewport is defined in opengl the clipping of the triangle is a bit skewed because one z-coordinate is on the positive side while the other is on the negative.

#### iii)
The purpose of the clipbox is to prevent opengl from rendering items which cannot be seen by the viewport. This also means that elements behind another element would normally be clipped as well.
