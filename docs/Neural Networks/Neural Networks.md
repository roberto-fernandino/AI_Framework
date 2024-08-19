![[Pasted image 20240814102848.png]]

***Layers***
---
> Layers in a neural network are fundamental building blocks that organize the network. Each layer consists of a set of neurons (or nodes) that perform computations and pass information to the next layer.
 
 #structure In this program we represent the layers as a Struct with two fields, **layers_vec** & **activation_vec**.
```rust
pub struct Layers<'a> {
    pub layers_vec: Vec<usize>,
    pub activation_vec: Vec<Activation<'a>>,
}
// Example
let layers = Layers::new(vec![2, 10, 1], vec![SIGMOID, SIGMOID, SIGMOID]);
```
Visually this is the network generated with this layers.
![[Pasted image 20240814103139.png]]
### Input Layer: 
This is the first layer that receives the raw data. It doesn't perform any computation but passes the input features to the next layer.
### Hidden Layers: 
These layers are situated between the input and output layers and perform computations on the data. Each hidden layer consists of multiple neurons that apply activation functions to the weighted sum of inputs.
### Output Layer:
This is the final layer that produces the network’s output. It typically uses a function like [softmax](https://www.google.com/search?client=firefox-b-d&q=fun%C3%A7%C3%A3o+softmax) (for classification tasks) or [linear activation](https://www.google.com/search?q=fun%C3%A7%C3%B5es+de+ativa%C3%A7%C3%A3o+lineares&client=firefox-b-d&sca_esv=ca10b7ff663b7daf&sca_upv=1&sxsrf=ADLYWILYm8UBzGoL2IiElYHKGuNBWKgRyQ%3A1723643681539&ei=Ibe8Zu_MIMPI1sQP7v-Q2Ac&ved=0ahUKEwiv4e2K0fSHAxVDpJUCHe4_BHsQ4dUDCA8&uact=5&oq=fun%C3%A7%C3%B5es+de+ativa%C3%A7%C3%A3o+lineares&gs_lp=Egxnd3Mtd2l6LXNlcnAiIGZ1bsOnw7VlcyBkZSBhdGl2YcOnw6NvIGxpbmVhcmVzMgUQIRigAUiCSlCpEFivSXAFeAGQAQCYAZQCoAH8IqoBBjAuMjYuM7gBA8gBAPgBAZgCIaACtCLCAgoQABiwAxjWBBhHwgINEAAYgAQYsAMYQxiKBcICChAjGIAEGCcYigXCAgQQIxgnwgIREC4YgAQYsQMY0QMYgwEYxwHCAgsQLhiABBixAxiDAcICCxAAGIAEGLEDGIMBwgIOEAAYgAQYsQMYgwEYigXCAgUQLhiABMICChAAGIAEGEMYigXCAgwQABiABBhDGIoFGArCAhQQLhiABBixAxiDARjHARiOBRivAcICEBAAGIAEGLEDGEMYgwEYigXCAggQABiABBixA8ICBRAAGIAEwgINEAAYgAQYsQMYQxiKBcICCBAAGIAEGMsBwgIIEAAYFhgeGA_CAgYQABgWGB7CAggQABiABBiiBMICBRAhGJ8FmAMAiAYBkAYJkgcGNS4yNS4zoAe8qAE&sclient=gws-wiz-serp) (for regression tasks) to generate predictions.


---
***Weights***
---
Weights refer to the parameters within a model that are learned from the training data. These weights adjust how the input data is transformed into output predictions.
Weights in neural networks determine the strength of the connection between neurons in adjacent layers. During training, these weights are adjusted to minimize the difference between the predicted output and the actual output.
	
#structure We represent weights in the code as
```rust 
Vec<Matrix> // [Matrix_a, Matrix_b... Matrix_n]
```
**[[Weights|See more detailed how this Matrix relation works here]]**
**[[Neural Netork.canvas|See visually in the canvas]]**
There are a bunch of algos to change the **Weights**, each one depending the situation and what you want to achieve.

---


*Biases*
---
Biases are kinda similar to [Weights](#Weights) but they are parameters added to the weighted sum of the inputs before passing it through the activation function different from weights that are multiplied. They allow the model to shift the activation function, providing additional flexibility. Biases help the model make better predictions when the input data doesn’t necessarily have to pass through the origin (i.e., when all inputs are zero).

#structure We represent biases in the code as

```rust 
Vec<Matrix> // [Matrix_a, Matrix_b... Matrix_n]
```

Every neuron in the Hidden Layer has a Bias value assigned to it.

---

*Data*
---
Data in this neural network is a Vector we use to store a "Traceback" of inputs and output every neuron in a layer made. 
Take a look at this example, this is a data Vec in the end of a FeedForward Network (FFN) action, we'll discuss FFN's later, but look:
Considering this is Networks has 2/3/1 [Layers](#Layers).
```rust
data: [
	Matrix { rows: 2, cols: 1, data: [[1.0], [1.0]] }, 
	Matrix { rows: 3, cols: 1, data: [[0.9999959003959371], [0.998332704623439], [0.0016032247432144918]] }, 
	Matrix { rows: 1, cols: 1, data: [[0.020126302745154437]] }
]
```
The first element of this **data** is a Matrix 2x1 previously transposed and represent the input data.
The second element is the output of the first **Hidden Layer**.
In case this network had more Layers in the **Hidden Layer** we would specify them here but as i said it is a 3 layer network.
The last element is the output.

---

***Learning Rate***
---
Learning Rate define how fast our model learns.
We define learning rate as a`f64`.
We adjust the values to control how "strenghly"" the Network will adjust the weights during the training process. Essentially, it determines the step size at each iteration as the model tries to minimize the loss function.
A Low **learning Rate** value result in a smaller updates and a more precise, though slower, convergence.
A High **Learning Rate** value can result in faster convergence bur risks overshooting the optimal weights or causing unstable training.
##### Example
Take a look at this example
This is a simple FFN network trained to learn [XOR](https://pt.wikipedia.org/wiki/Ou_exclusivo)
Consider this as input 
```rust
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];
```
This is the output of the Network after training with learning rate **0.5**, considered a medium/high **Learning Rate**.
![[Pasted image 20240817111710.png]]

> [!NOTE]
>  This test was made using ReLu, using others activation functions may vary the max **Learning Rate** values we can use.

The [Weights](#weights) were changed to much and the solution was not properly found.
With very large updates, the network can deviate greatly from the optimal values, resulting in outputs that can be close to zero due to large adjustments in weights that do not converge to useful values.

---
Reasons for Learning Rate Issues
--
1. **Updates of Very Large Weights**

    **Effect:** A high learning rate makes the weight updates very large. This can cause the network to "sack" from one side to the other instead of slowly adjusting and finding the optimal solution.
    **Consequence:** With very large updates, the network can deviate too much from the optimal values, resulting in outputs that can be close to zero due to large adjustments in weights that do not converge to useful values.

2. **The Problem of Divergence**

    **Effect:** If the learning rate is too large, the network can become unstable, leading to a divergence of the weight values. Instead of minimizing the error, the network can increase the error continuously.
    **Consequence:** This can cause the network to produce outputs that look "fixed" and do not change, or that do not fit well to the desired pattern.

3. **Explosion of Gradients**

    **Effect:** In deep networks, a high learning rate can exacerbate the problem of gradient explosion, where the gradients calculated during training become very large.
    **Consequence:** This can cause an excessive update of weights, resulting in activation values that are too large or too small, and can cause the network to produce inadequate outputs or even NaN.

4. **Adjustments Not Precise**

    **Effect:** The network may not have enough time to adjust the weights correctly when the learning rate is too high. Weights may be adjusted away from the correct solution before the network has a chance to stabilize.
    **Consequence:** This can result in a network that does not learn the desired pattern and continues to produce constant outputs as zero.

Now adjusting the learning rate to **0.01** this is the output we get
![[Pasted image 20240817112607.png]]
See how much closer to the expected output we are now?
Choose the right learning rate is crucial for training efficiency and accuracy

---
*Activation Functions*
---
In an [artificial neural network](https://en.wikipedia.org/wiki/Artificial_neural_network "Artificial neural network"), an **activation function** is a critical component that determines the output of each node (or neuron) based on the weighted sum of its inputs. These functions introduce non-linearity into the network, enabling it to model complex patterns and relationships in data that a simple linear transformation could not capture. Activation functions are responsible for deciding whether a neuron should be "activated" or not, influencing how information flows through the network and ultimately how it learns to make predictions.

By transforming input signals, activation functions allow neural networks to approximate almost any function, making them powerful tools in fields like image recognition, natural language processing, and decision-making systems.

---

**Neural Networks types**
---
1. **FFN** FeedForward Neural Network
	1. Consists typically of three types of [layers](#layers).
		1. [[#Input Layer]]
		2. [[#Hidden Layers]]
		3. [[#Output Layer]]
	2. [[#Activation Functions]]: Nodes in the hidden and output layers use activation functions to introduce non-linearity into the network. Common activation functions include ReLU, Sigmoid, and Tanh.
	3. [[#Back Propagation]]: We almost always use BackPropagation algorithms to train a FFN.
		

---
**Algorithms**
---
### FeedForward
---
![[Pasted image 20240819090903.png]]
It's a method of a Network Struct.
#### **Input**
We receive a Vector of 64-bit floating numbers as input data and returns a Vector of 64-bit floating numbers.
The inputs Vector represents the input layer.
Every f64 is "inputted into the input layer" but more precisely they're the input layer because the input layer doesn't do anything with the data, they don't have any [[#Activation Functions]] , [[#Weights]] and [[#Biases]].

#### **Algorithm**
First
1. We check the length of the `inputs` Vector to check if they're the same size as the input layer.
2. Create a matrix from the `inputs` and transpose it.
	Example:
```rust 
     let inputs = [1.0, 0.0];
     asserteq!(self.layers.layers_vec[0], inputs.len() - 1); // True
```
This inputs  will result in a matrix like this after the transpose
| **1.0**  | `Input Neuron 1`
| **0.0** | `Input Neuron 2`

3. Create a data Vector which will store all the data the Network create and push       this matrix, we'll give the name `current` because this Matrix will further change to represent the output of every layer.
	![[Pasted image 20240819093339.png]]
	 Data will look like this atm
	 ```rust
	 [Matrix { rows: 1, cols: 2, data: [1.0, 0.0] }]
	 ```
4. Now we iterate through the layers.
     For every layer in the network we'll create a new current that will be the result a matrix with every element being the result of 
     **[[#Activation Functions|act_func_of_that_layer]]([[#Weights|weights_of_that_layer]] * current_matrix + [[#Biases|bias_of_that_layer]])**
     This current matrix represents the IO of a layer.
     Last `current` = Input of actual layer.
	 
5. Return the **last** current matrix that is output of [[#Output Layer]].

#### **Output**
The output is the "Network Response" itself, it's the data the network changed in the [[#Hidden Layers]] and [[#Output Layer]]. 

---

### Back Propagation
---
The `back_propagation` function adjusts the weights and biases of the network by calculating and propagating the error backwards through the layers of the neural network.
1. **Target Size Verification**: We first check if the size of the `targets` vector matches the size of the output layer (i.e., the number of neurons in the output layer). If not, the function panics, as the targets must correspond to the expected output size.
```rust 
if targets.len()!= self.layers.layers_vec[self.layers.layers_vec.len()-1] { 
panic!("Targets must have the same size as the last layer"); 
}
```
2. **Convert Outputs and Targets to Matrices**: The `outputs` and `targets` vectors are converted into matrices and transposed. This allows us to perform matrix operations during the backpropagation process. 
```rust 
let outputs = Matrix::from(vec![outputs]).transpose();
let targets = Matrix::from(vec![targets]).transpose();
```
3. **Initial Error Calculation**: The initial `errors` matrix is computed by subtracting the `outputs` from the `targets`. This represents the error between the network's output and the expected output (targets).
```rust
let mut errors = targets.subtract(&outputs);
```
4. **Initial Gradient Calculation**: The gradient is calculated using the derivative of the activation function of the output layer, applied to the `outputs`. This gradient will be used to adjust the weights and biases in subsequent layers.
```rust
let mut gradients: Matrix = outputs.map(&self.layers.activation_vec.last().unwrap().derivative);
```
5. **Iterate Through Layers in Reverse**: We iterate through the layers in reverse order (from output layer to input layer) to propagate the error backward:
	1. **Gradient Adjustment**: For each layer, the gradients are adjusted by applying the element-wise product of the gradients and the `errors`, scaled by the learning rate.
		 ```rust
		 gradients = gradients
	    .dot_multiply(&errors)
	    .map(&|x| x * self.learning_rate);
		 ```
	2. **Update Weights and Biases**: The weights of the current layer are updated by adding the product of the `gradients` and the transpose of the data (inputs to that layer). The biases are updated by adding the `gradients`.
	  ```rust
	  self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));  self.biases[i] = self.biases[i].add(&gradients);
	  ```
	3. **Calculate Errors for Previous Layer**: The `errors` for the previous layer are calculated by multiplying the transposed weights of the current layer by the current `errors`. This propagates the error back to the previous layer.
	```rust
	errors = self.weights[i].transpose().multiply(&errors);
	```
	4. **Calculate Gradients for Previous Layer**: The gradients for the previous layer are recalculated by applying the derivative of the activation function of that layer to the data (inputs) of the layer.
	```rust
		gradients = self.data[i].map(self.layers.activation_vec[i].derivative);
	```
	5. **Completion**: After iterating through all the layers, the `weights` and `biases` of the network have been updated. This process of adjusting weights and biases minimizes the error and allows the network to learn from the data.
---