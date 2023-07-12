# LLM-CHATBOT-RS

A ChatGPT clone application utilising HuggingFace's Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin LLM model (can be replaced with the model of your choice/preference), built with Rust-Leptos.

After installation of dependencies, including tailwindcss in the package.json, run the following command to apply styling to the project:

        npx tailwindcss -i ./input.css -o ./style/output.css


#### Note
- Currently, this project is meant to be run locally.
- The model must be present in the root directory and should be declared in the .env (whose sample has been provided as .env.sample).