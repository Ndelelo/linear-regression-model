# Linear Regression Model in Rust

## Introduction
Linear regression is one of the fundamental building blocks of machine learning, serving as an excellent starting point for understanding how AI models learn from data. In this assignment, i was implementing a simple linear regression model using Rust and the Burn deep learning framework to predict values for the linear function y = 2x + 1.

Below are the steps and instructions, from setting up the development environment to training and evaluating the model.

## Prerequisites
Before starting, ensure the following applications are installed:

- **Rust** - A programming language known for its performance, memory safety, and concurrency. [Download Rust](https://www.rust-lang.org/)
- **Rust Rover IDE** - An IDE for Rust development by JetBrains, providing features like code completion, debugging, and project management. [Download Rust Rover](https://www.jetbrains.com/rustrover/)
- **Git** - A version control system for tracking code changes, collaboration, and project management. [Download Git](https://git-scm.com/)

## Create a New Rust Project
Open PowerShell Terminal and create a new Rust project using Cargo:

```sh
cargo new linear_regression_model
cd linear_regression_model
```

Open the project in Rust Rover:

```sh
rustrover .
```

## Configure Dependencies
Edit the `Cargo.toml` file and update the `[dependencies]` section with the provided dependencies for the project.

## Connect Rust Rover to GitHub

1. Open GitHub and create a new repository called `linear-regression-model`.
2. Enable version control in Rust Rover:
   - Go to **VCS > Enable Version Control Integration** and select **Git**.
3. Open the terminal and navigate to the project directory.
4. Create a `.gitignore` file to exclude unnecessary files:

```plaintext
/target/
Cargo.lock
**/*.rs.bk
*.pdb
.idea/
.vscode/
*.iml
.DS_Store
.env
.env.local
```

5. Link and commit the Cargo project to GitHub:

```sh
git init
git remote add origin https://github.com/Ndelelo/linear-regression-model.git
git add .
git branch -M main
git commit -m "Initial commit"
git push -u origin main
```

## Implementing the Linear Regression Model

1. Open the terminal and create `data.rs` and `model.rs` under the `src` directory:

```sh
touch src/data.rs src/model.rs
```

2. Implement data generation in `data.rs`.
3. Define the linear regression model in `model.rs`.
4. Train the model in `main.rs`.

## Run the Project
Click the **Run** button in Rust Rover to build and run the project. Monitor the console output for training progress and final model parameters.

## Approach and Challenges

### Approach:
1. **Setting up the environment**: Installed Rust, Rust Rover IDE, and Git.
2. **Creating a new Rust project**: Used Cargo to generate a new project and configured dependencies in `Cargo.toml`.
3. **Implementing the model**:
   - Generated synthetic data in `data.rs`.
   - Defined the linear regression model in `model.rs`.
   - Trained the model in `main.rs` using the `burn` library (version 0.16.0).
4. **Connecting to GitHub**: Linked the project to a GitHub repository for version control.
5. **Running and evaluating the model**: Monitored the output to ensure correct predictions.

### Challenges Faced:
- **Understanding the burn library**: Limited documentation required additional research to correctly implement the linear regression model.
- **Dependency management**: Ensuring compatibility between `burn` version 0.16.0 and other dependencies in `Cargo.toml` proved to be challenging and time-consuming to fix.
- **Debugging training issues**: Adjusting hyperparameters and debugging errors in model training to improve accuracy.
- **Handling Numeric Computation Efficiently**: Rust doesn’t have built-in NumPy-style operations, so working with matrices and vectors was tougher and caused errors nonstop.
- **Git Issues**: Large files detected, preventing the push operation from completing successfully until the creation of a `.gitignore` file to exclude unnecessary files.

## Resources Used

### AI Tools:
- **Claude**
- **Microsoft Copilot**
- **ChatGPT**

### Tutorials:
- **YouTube**

### Learning Process:
I received significant assistance from AI in the following areas:

- **Project Structure and Setup**—Guidance on creating a Rust project using Cargo, setting up dependencies, and linking GitHub.
- **GitHub Integration**—Step-by-step instructions for initializing Git, creating `.gitignore`, and pushing code to GitHub after some errors of pushing code to `master` instead of `main`.
- **README Formatting** – Structuring the documentation with clear sections, Markdown formatting, and best practices.
- **Approach and Challenges**—AI helped in articulating most problem-solving processes, potential difficulties, and ways to address them.

## Why the Implementation Failed

Despite following the outlined approach, the implementation did not successfully train a linear regression model using the `burn` library version 0.16. The primary reasons for failure include:

- **Lack of Comprehensive Documentation** - The `burn` library's official documentation is still evolving, making it difficult to find clear examples for implementing a linear regression model.
- **Incompatibility Issues** - Some dependencies did not work well together, causing errors in compilation and execution.
- **Model Training Issues** - Despite following standard linear regression training steps, the model did not converge properly, possibly due to incorrect hyperparameter tuning or issues with data representation.

## Lessons Learned

Through this experience, some key lessons were gained:

- **Library Selection Matters**: Choosing a well-documented and community-supported library can significantly ease implementation.
- **Thorough Testing is Crucial**: Testing model components step by step can help in identifying issues early.
- **Debugging Requires Patience**: AI model implementation requires iterating over multiple configurations and parameters.
