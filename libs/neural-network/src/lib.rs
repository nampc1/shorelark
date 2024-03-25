pub struct NeuralNetwork {
    depth: usize,
    layers: Vec<Layer>,
}

struct Layer {
    size: usize,
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    output_weights: Vec<f32>,
}

impl NeuralNetwork {
    fn new(depth: usize, layer_params: usize, ) -> Self {
        NeuralNetwork {
            depth: 0,
            layers: Vec::new(),
        }
    }

    // why mut T (binding - pattern syntax) but not T: &mut (type) https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t
    // binding is local to the function
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // fold is good when you need to produce a single value from a collection
        self.layers
            .iter()
            .fold(inputs, |acc,  layer| layer.propagate(acc))
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // better to avoid writing loops on your own, use iter and iter adaptor
        // Read more: vec with capacity and iter
        // Vec starts empty, with every push Vec is moved to another place with enough space => inefficient
        self.neurons.iter().map(|neuron| neuron.propagate(&inputs)).collect()
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.output_weights.len());

        // array[i] always perform a bounds check 
        let mut output = inputs // iterator is lazy
            .iter()
            .zip(&self.output_weights)
            .map(|(&input, &weight)| input * weight)
            .sum::<f32>(); // ::<> turbofish - provides explicit generic args when compiler can't infer. https://techblog.tonsser.com/posts/what-is-rusts-turbofish

        output += self.bias;

        output.max(0.0)
    }
}


// node value = activation_func(input val * weight + bias)

#[cfg(test)]
mod tests {

}
