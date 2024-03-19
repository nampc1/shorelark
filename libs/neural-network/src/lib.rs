pub struct NeuralNetwork {
    depth: usize,
    layers: Vec<String>,
}

impl NeuralNetwork {
    fn new() -> Self {
        NeuralNetwork {
            depth: 0,
            layers: Vec::new(),
        }
    }

    fn add_layer() {
        // increase depth
        // add a layer
    }

    fn propagate() {
        todo!()
    }
}

pub struct Layer {
    neurons: Vec<Neuron>,
}

pub struct Neuron {
    bias: f32,
    output_weights: Vec<f32>,
}

impl Neuron {
    fn new(bias: f32, synapse_count: usize) -> Self {
        let mut output_weights: Vec<f32> = vec![];
        
        for i in 0..synapse_count { // todo: randomize output weight and improve loop
            output_weights.push((i as f32) * 0.2);
        }
        
        Neuron {
            bias,
            output_weights
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NeuralNetwork, Neuron};

    #[test]
    fn test_neural_network() {
        let neural_net = NeuralNetwork::new();
        assert_eq!(neural_net.depth, 0);
        println!("i'm here {}", neural_net.depth);
    }
    
    #[test]
    fn test_neuron() {
        let neuron = Neuron::new(0.3, 10);
        println!("{:?}", neuron.output_weights);
    }
}
