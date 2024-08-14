For this layer in this case we'll have weights as a 
```rust 
Matrix::random_less_one_to_one(rows: 3, cols: 2)
```
Each row represents the weights for a single neuron in the next layer, and each column represents a connection from a neuron in the current layer.

| x11     | x12     |
| ------- | ------- |
| **x21** | **x22** |
| **x31** | **x32** |

This will be pushed to the Vector storing all the weights.
