# Rust Binary Tree Implementation

I am developing this binary tree implementation with a few goals in mind.
1. I want to truly learn Rust 
2. I want to use this code to solve Project Euler questions that require a Binary Tree
3. I want to learn how Rust handles Memory so that I can implement this in many different ways.

## Current Approach
My current approach is to have the main *Tree* object own all the memory for the tree.
As opposed to each parent owning it's children and using the Reference counter/Reference cell.

Again, I am doing this purely for my own education of the Rust library. 
Considering I could easily use the pre-built "graph-builder" crate to create a binary tree, this really only has educational purposes.