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
This happens when we try to render an element outside the bounds of the clip box. As stated in the opengl guide for this exercise the clip box is a 2x2x2 cube. This means the bounds for all axises are from -1.0 to 1.0, therefore the coordinates for the z-axis are out of bounds. Since the z-axis is also the axis in which the viewport is defined in opengl the clipping of the triangle is a bit skewed because one z-coordinate is on the positive side while the other is on the negative.

#### iii)
The purpose of the clipbox is to prevent opengl from rendering items which cannot be seen by the viewport. This also means that elements behind another element would normally be clipped as well.

### b)

![
    The resulting black screen of task 2b
](images/2b.png)

#### i)

What happens is that we get a black/blank screen because the triangle is not getting rendered.

#### ii)

This happens because the path between the vertices defined by the indices has no space to fill with color. Therefore we get a black window.

#### iii)

This happens when opengl "traverses" the path of the vertices defined by the indices such that the triangle is not enclosed at the left side of the path. This is because opengl fills this enclosed space with the color therefore rendering the triangle. When we swap the place of two indices we modify the traversal such that the enclosed space is on the right side of the direction of travel. Because of this it does not render. However if we pop the last index and push it to the front (for a single triangle in this case) it would still render because the direction of the traversal would stay on the left side. E.g [2, 0, 1] and [1, 2, 0].

### c)

#### i)

If you did not clear the depth buffer in the case of a sphere moving rightward we would see a straight line being drawn. This is because if the buffer is not cleared the previous drawing instructions will remain.

#### ii)

The fragment shader can be executed multiple times for the same pixel if there are elements behind or in front of eachother on the same pixel. In that case the fragment shader may execute multiple times for the pixel by processing multiple fragments.

#### iii)

The two most commonly used type of shaders is the vertex shader and the fragment shader.
The responsibility of the vertex shader is calculating/determining the correct possition for each vertex in the scene.
The responsibility of the fragment shader is computing the color of the fragment being rendered.

#### iv)

It is common to use an index buffer instead of the order of vertices for optimization purposes. Since the indices determine which vertices opengl will draw to you can reuse the vertices by specifying the order with indices. The opengl book states that this is more significant for larger scenes where you can save computing power and data size.

#### v)

A situation where we would not pass in a null pointer as the last argument for the 
```rust
gl::VertexAttribPointer()
```
is when we have a buffer where we store multiple types of data along with e.g coordinates. This additional data along with the coordinates may be stored after the coordinates, therefore we need a pointer to the first instance of our additional data and the pass a non zero value into this function parameter.

### d)

#### i)

![
    The triangle before inverting
](images/2di.png)

![
    The triangle after inverting
](images/2di-inverted.png)

I achieved this inverting by multiplying the vector coordinates in the vertex shader with a factor of -1.

#### ii)

![
    The triangle after getting a new colour 
](images/2dii.png)

I achieved this by modifying the output vector in the fragment shader with some float between 0 and 1 for each value.
