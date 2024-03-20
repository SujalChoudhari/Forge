# Forge: Automate Tasks with Ease
![forgeLogo](https://github.com/SujalChoudhari/Forge/assets/85174767/efdd0846-2bc9-4b59-9dbd-4747b6d7a63c)

## Introduction

Forge is a powerful command-line tool written in Rust that simplifies the execution of tasks based on configuration files written in YAML format. It allows you to define conditions, dependencies, and commands within these files, enabling you to automate repetitive workflows or execute tasks conditionally depending on your operating system or file changes.

**Forge your development workflow**: Imagine a blacksmith's forge, where raw materials are transformed into powerful tools. Forge, the command-line tool, empowers you to do the same with your development process. Craft streamlined workflows, automate repetitive tasks, and focus on what matters most - building your dream project.

### Getting Started

1. **Installation:**
   - **Pre-built Binaries:** Download pre-built binaries for your platform from [Releases](https://github.com/SujalChoudhari/Forge/releases)

2. **Create a Forge File:**
   - Create a YAML file named `ForgeFile` in your project (root) directory.

## Forge File Structure

A Forge file consists of sections that define various aspects of task execution. Here's a breakdown of the available sections:

- You can define multiple recipes in your Forge file, each with a unique name.
- A recipe is a section that specifies how a particular task should be executed. It can have the following subsections:

  * **`on` (Optional):**
    - An array of strings that determines on which operating systems the recipe should run. Valid options include:
      - `"linux"`
      - `"win"`
      - `"mac"`
      - `"all"` (to run on all supported systems)
  * **`detect` (Optional):**
    - An array of file patterns that trigger the recipe when a file matching these patterns changes. You can use the wildcard character `*` to match any filename.
  * **`always` (Optional):**
    - A boolean flag (default: `false`). If set to `true`, the recipe runs regardless of file changes or operating system conditions.
  * **`run` (Required):**
    - An array of strings representing the commands to execute for this recipe. You can use variables within commands (explained in the Variables section).


## Variables

Forge supports variables that can be used within commands for dynamic behavior. Here's how to define and use variables:

- **In `vars` section:**
  - Define variables under the `vars` section of a recipe or the global Forge file:

    ```yaml
    vars:
      my_variable: some value
      my_var: [value, in, a, matrix]
      ```

- **Using Variables:**
  - Access variables within commands using the following syntax:
    - `{variable_name}`: For string variables.
    - `{$variable_name}` (with curly braces): For getting the index number for that file.
  - Default variables
    - `fileName`: Get the name for current file in execution.
    - `fileNameExt`: Get the name with extention for current file in execution.
    - `fileExt`: Get the extention for current file in execution.
    - `fileDir`: Get the name for directory of the file in execution.
    - `filePath`: Get the entire path of the file.
    - `os`: Current os the code is running on.


**Example Forge File**

```yaml
# This is the default recipe
forge:
  on: [linux, win]
  run: echo "Hello, world!"

# Another recipe named "test"
test:
  detect: ["*.txt"]  # Runs when any .txt file changes
  run:
    - cat {filePath}  # Prints the content of the changed file
    - echo "Test completed"

hack:
  always: true
  run:
    - linux # run below commands if os is linux
    - ping example.com
    
    - win # run below commands only if os is windows
    - ping sujal.xyz

commit:
  detect: ["src/","*.js", "Cargo.*"] # a file has to satisfy one or more condition for the forge to detect.
  run:
  - git add -A
  - git commit -m {message}  # can declare message as `forge commit --message="Testing"` if not it will be asked during execution.
  - git push
```

## Contributing

We welcome contributions to Forge! Here's how you can get involved:

- **Report bugs:** If you encounter issues, please create a new issue on our GitHub repository: [Repo](https://github.com/SujalChoudhari/Forge)
- **Suggest improvements:** Share your ideas for enhancing Forge through pull requests.
- **Pull requests:** Fork the repository, make your changes, and submit a pull request for review.

## License

This project is licensed under the MIT License. You can find the full license text in the [`LICENSE`](https://github.com/SujalChoudhari/Forge/blob/main/LICENSE) file within the project directory.
