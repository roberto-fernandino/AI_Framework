![[Pasted image 20240814102848.png]]

_**Layers**_
---
> Layers in a neural network are fundamental building blocks that help the network learn from data. Each layer consists of a set of neurons (or nodes) that perform computations and pass information to the next layer.

In this program we represent the layers as a Vector of usize integers.

```rust
layers: Vec<usize> = vec![2,3,2]
```
Visually this is the network generated with this layers.
![[Pasted image 20240814103139.png]]

**Input Layer**: 
	This is the first layer that receives the raw data. It doesn't perform any computation but passes the input features to the next layer.
	
**Hidden Layers**: 
	These layers are situated between the input and output layers and perform computations on the data. Each hidden layer consists of multiple neurons that apply activation functions to the weighted sum of inputs.
	 
**Output Layer**:
	This is the final layer that produces the network’s output. It typically uses a function like [softmax](https://www.google.com/search?client=firefox-b-d&q=fun%C3%A7%C3%A3o+softmax) (for classification tasks) or [linear activation](https://www.google.com/search?q=fun%C3%A7%C3%B5es+de+ativa%C3%A7%C3%A3o+lineares&client=firefox-b-d&sca_esv=ca10b7ff663b7daf&sca_upv=1&sxsrf=ADLYWILYm8UBzGoL2IiElYHKGuNBWKgRyQ%3A1723643681539&ei=Ibe8Zu_MIMPI1sQP7v-Q2Ac&ved=0ahUKEwiv4e2K0fSHAxVDpJUCHe4_BHsQ4dUDCA8&uact=5&oq=fun%C3%A7%C3%B5es+de+ativa%C3%A7%C3%A3o+lineares&gs_lp=Egxnd3Mtd2l6LXNlcnAiIGZ1bsOnw7VlcyBkZSBhdGl2YcOnw6NvIGxpbmVhcmVzMgUQIRigAUiCSlCpEFivSXAFeAGQAQCYAZQCoAH8IqoBBjAuMjYuM7gBA8gBAPgBAZgCIaACtCLCAgoQABiwAxjWBBhHwgINEAAYgAQYsAMYQxiKBcICChAjGIAEGCcYigXCAgQQIxgnwgIREC4YgAQYsQMY0QMYgwEYxwHCAgsQLhiABBixAxiDAcICCxAAGIAEGLEDGIMBwgIOEAAYgAQYsQMYgwEYigXCAgUQLhiABMICChAAGIAEGEMYigXCAgwQABiABBhDGIoFGArCAhQQLhiABBixAxiDARjHARiOBRivAcICEBAAGIAEGLEDGEMYgwEYigXCAggQABiABBixA8ICBRAAGIAEwgINEAAYgAQYsQMYQxiKBcICCBAAGIAEGMsBwgIIEAAYFhgeGA_CAgYQABgWGB7CAggQABiABBiiBMICBRAhGJ8FmAMAiAYBkAYJkgcGNS4yNS4zoAe8qAE&sclient=gws-wiz-serp) (for regression tasks) to generate predictions.
```rust
layers[0] // Input layer
// Hidden layers are all other layers between these 2
layers[layers.len() - 1] // Output Layer
```
---
***Weights***
---
Weights refer to the parameters within a model that are learned from the training data. These weights adjust how the input data is transformed into output predictions.
Weights in neural networks determine the strength of the connection between neurons in adjacent layers. During training, these weights are adjusted to minimize the difference between the predicted output and the actual output.

	We represent weights in the code as a:
```rust 
Vec<Matrix> // [Matrix_a, Matrix_b... Matrix_n]
```
For every layer in the network we create a ==Matrix== with **rows = layer_size + 1** & **cols = layer_size** and we push to the weights Vector.
	**[[Neural Netork.canvas|See in the canvas]]**

There are a bunch of algos to change the **Weights** each one with a use case. 

---
*Biases*
---

Biases are kinda similar to [Weights](#Weights) but they are parameters added to the weighted sum of the inputs before passing it through the activation function different from weights that are multiplied. They allow the model to shift the activation function, providing additional flexibility. Biases help the model make better predictions when the input data doesn’t necessarily have to pass through the origin (i.e., when all inputs are zero).