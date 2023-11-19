# Overview

These days, I am working on LocalAI project new Python and Rust backend. So, I hit lots of issues. And one of these issues is this one: **how to read files in Rust?**

Here is the senario, we are going to define a CNN model and it's parameters with Rust. After that we need to load the checkpoint file to get the parameters. So, we need to read the checkpoint file in Rust. And after all of these, we can use the model to do inference.


# The example of the definition of the MNINST model

Here is an example of the model definition:

```rust
impl<B: Backend> MNINST<B> {
    pub fn new(model_name: &str) -> Self {
        let conv1 = ConvBlock::new([1, 8], [3, 3]); // 1 input channel, 8 output channels, 3x3 kernel size
        let conv2 = ConvBlock::new([8, 16], [3, 3]); // 8 input channels, 16 output channels, 3x3 kernel size
        let conv3 = ConvBlock::new([16, 24], [3, 3]); // 16 input channels, 24 output channels, 3x3 kernel size
        let hidden_size = 24 * 22 * 22;
        let fc1 = nn::LinearConfig::new(hidden_size, 32)
            .with_bias(false)
            .init();
        let fc2 = nn::LinearConfig::new(32, NUM_CLASSES)
            .with_bias(false)
            .init();

        let dropout = nn::DropoutConfig::new(0.5).init();

        let instance = Self {
            conv1: conv1,
            conv2: conv2,
            conv3: conv3,
            dropout: dropout,
            fc1: fc1,
            fc2: fc2,
            activation: nn::GELU::new(),
        };
        let state_encoded: &[u8] = &std::fs::read(model_name).expect("Failed to load model");
        let record = BinBytesRecorder::<FullPrecisionSettings>::default()
            .load(state_encoded.to_vec())
            .expect("Failed to decode state");

        instance.load_record(record)
    }

    /// Applies the forward pass of the neural network on the given input tensor.
    ///
    /// # Arguments
    ///
    /// * `input` - A 3-dimensional tensor of shape [batch_size, height, width].
    ///
    /// # Returns
    ///
    /// A 2-dimensional tensor of shape [batch_size, num_classes] containing the output of the neural network.
    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        // Get the dimensions of the input tensor
        let [batch_size, height, width] = input.dims();
        // Reshape the input tensor to have a shape of [batch_size, 1, height, width] and detach it
        let x = input.reshape([batch_size, 1, height, width]).detach();
        // Apply the first convolutional layer to the input tensor
        let x = self.conv1.forward(x);
        // Apply the second convolutional layer to the output of the first convolutional layer
        let x = self.conv2.forward(x);
        // Apply the third convolutional layer to the output of the second convolutional layer
        let x = self.conv3.forward(x);

        // Get the dimensions of the output tensor from the third convolutional layer
        let [batch_size, channels, height, width] = x.dims();
        // Reshape the output tensor to have a shape of [batch_size, channels*height*width]
        let x = x.reshape([batch_size, channels * height * width]);

        // Apply dropout to the output of the third convolutional layer
        let x = self.dropout.forward(x);
        // Apply the first fully connected layer to the output of the dropout layer
        let x = self.fc1.forward(x);
        // Apply the activation function to the output of the first fully connected layer
        let x = self.activation.forward(x);

        // Apply the second fully connected layer to the output of the activation function
        self.fc2.forward(x)
    }

    pub fn inference(&mut self, input: &[f32]) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        // Reshape from the 1D array to 3d tensor [batch, height, width]
        let input: Tensor<B, 3> = Tensor::from_floats(input).reshape([1, 28, 28]);

        // Normalize input: make between [0,1] and make the mean=0 and std=1
        // values mean=0.1307, std=0.3081
        // Source: https://github.com/pytorch/examples/blob/54f4572509891883a947411fd7239237dd2a39c3/mnist/main.py#L122
        let input = ((input / 255) - 0.1307) / 0.3081;

        // Run the tensor input through the model
        let output: Tensor<B, 2> = self.forward(input);

        // Convert the model output into probalibility distribution using softmax formula
        let output = burn::tensor::activation::softmax(output, 1);

        // Flatten oupuut tensor with [1,10] shape into boxed slice of [f32]
        let output = output.into_data().convert::<f32>().value;

        Ok(output)
    }
}
```


# Read files in Rust

Here are two solutions we can give to solve reading files in Rust. The `include_bytes!` macro and `std::fs::read` function are both used to read files in Rust, but they work in different ways and are used in different scenarios:


## 1. `include_bytes!` macro

This is a macro that includes a file's contents into the binary at compile time. The file is specified as a string literal, and it must exist at compile time. The contents of the file are included as a byte array (`&[u8]`). This is useful when you have small files that you want to bundle with your binary, and you know at compile time what those files will be.


## 2. `std::fs::read` function

This is a function that reads a file's contents at runtime. The file is specified as a string, which can be a string literal or a string variable. The contents of the file are returned as a `Vec<u8>`. This is useful when you need to read files that are specified at runtime, or large files that you don't want to include in your binary.


# Summary

As you can see, with `include_bytes!` the contents are included in the binary at compile time, so you can't use it to read files that are specified at runtime. With `std::fs::read` the contents are read at runtime, so you can't use it to read files that are specified at compile time.
