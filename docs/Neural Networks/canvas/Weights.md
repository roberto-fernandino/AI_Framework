### Definition

Weights exist between every layer; they represent the link and the strength of the connection between two neurons.

```rust 
Matrix::random_less_one_to_one(rows: 3, cols: 2) // Matrix we use to represent weights.
```

Each row represents the weights for a single neuron in the next layer, and each column represents a connection to a neuron from the current layer.

| x11     | x12     |
| ------- | ------- |
| **x21** | **x22** |
| **x31** | **x32** |

Every row (**x21**, **x22**, etc.) represents the weights of a unique neuron in the **current layer**, with each value corresponding to the connection strength to the neurons in the **next layer**..
### Example

Let's assign symbolic values to the weight matrix, simulating the bonding forces between neurons:

```rust 
// Each inner Vector represents a weight 
Matrix::new(vec![ 
	vec![0.5, -0.7],  
	vec![-0.2, 0.9], 
	vec![0.8, -0.4],
])
```

This would result in a matrix like this:

| 0.5      | -0.7     |
| -------- | -------- |
| **-0.2** | **0.9**  |
| **0.8**  | **-0.4** |
See the applied values Canvas bellow ![[Neural Netork.canvas]]


After that we'll 