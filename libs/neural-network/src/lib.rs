use rand::Rng;

pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    output_weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

impl NeuralNetwork {
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let result_layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self {
            layers: result_layers,
        }
    }

    // why mut T (binding - pattern syntax) but not T: &mut (type) https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t
    // binding is local to the function
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // fold is good when you need to produce a single value from a collection
        self.layers
            .iter()
            .fold(inputs, |acc, layer| layer.propagate(acc))
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        // better to avoid writing loops on your own, use iter and iter adaptor
        // Read more: vec with capacity and iter
        // Vec starts empty, with every push Vec is moved to another place with enough space => inefficient
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();

        Self { neurons }
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

    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.00))
            .collect();

        Self {
            bias,
            output_weights: weights,
        }
    }
}
