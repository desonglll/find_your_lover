# Find Your Lover - Neural Network-Based Matchmaking Scoring System

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Introduction

**Find Your Lover** is a Rust-based command-line application that simulates a simple neural network to evaluate and score matchmaking candidates based on different attributes. Users can input various attributes for each candidate, and the program calculates a weighted score for each candidate to help in making informed decisions.

## Features

- Customizable attributes for each matchmaking candidate.
- Weight-based scoring system that prioritizes different aspects like appearance, personality, financial stability, and more.
- Continuous input mode allowing users to evaluate multiple candidates in one session.
- Unit tests to ensure the reliability of the scoring algorithm.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or higher

### Steps

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/find-your-lover.git
   cd find-your-lover
   ```

2. **Build the project:**

   ```bash
   cargo build --release
   ```

3. **Run the application:**

   ```bash
   cargo run
   ```

## Usage

When you run the application, you will be prompted to enter the name, age, and various attribute scores for each matchmaking candidate. The program will then calculate and display a total score out of 100 for the candidate.

### Example

```bash
请输入相亲对象的名字：
> 李华
请输入李华的年龄：
> 28
请输入相亲对象的各项评分（0-100）：
外貌：
> 85
性格：
> 90
经济能力：
> 75
幽默感：
> 80
兴趣爱好：
> 70
家庭背景：
> 65
健康状况：
> 95
生活习惯：
> 70
职业：
> 80
教育背景：
> 90
李华的总评分为：83.75/100

是否继续输入下一个相亲对象？(y/n)
> n
```

## Configuration

### Attribute Weights

The weights for each attribute are predefined in the code as follows:

- Appearance: 0.15
- Personality: 0.20
- Financial Stability: 0.15
- Sense of Humor: 0.10
- Common Interests: 0.10
- Family Background: 0.10
- Health: 0.05
- Lifestyle: 0.05
- Career: 0.05
- Education: 0.05

If you want to customize the weights, you can modify the `weights` vector in the `calculate_score` method of the `Candidate` struct.

## Testing

The project includes unit tests to ensure that the scoring mechanism works as expected. You can run the tests using the following command:

```bash
cargo test
```

The test suite includes a sample candidate with predefined attribute scores. The test ensures that the calculated score falls within an expected range.

## Contributing

We welcome contributions to the project! Here’s how you can help:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes with clear commit messages.
4. Push your branch to your fork.
5. Open a pull request with a detailed description of your changes.

### Coding Style

Please follow the Rust coding conventions and ensure your code is formatted according to the standard Rust formatting tool:

```bash
cargo fmt
```

### Running Linter

To maintain code quality, you can run `clippy` to check for common mistakes and improvements:

```bash
cargo clippy
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for their amazing tools and libraries.
- Inspired by the idea of combining simple neural networks with real-world applications.

## Contact

If you have any questions, suggestions, or feedback, feel free to reach out:

- GitHub: [desonglll](https://github.com/desonglll)
- Email:  lindesong666@gmail.com